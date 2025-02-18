use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Expr, ExprPath, Lit, LitStr, Meta, Type};

use crate::{check_duplicate, is_option, is_vec};

#[derive(Clone)]
pub(crate) enum ConvertFieldMap {
    Map(Expr),
    FieldFn(ExprPath),
    StructFn(ExprPath),
    Suffix(TokenStream),
}

impl ConvertFieldMap {
    pub(crate) fn gen_suffix(ty: &Type) -> Self {
        let suffix = {
            if is_vec(ty) {
                quote::quote! {.into_iter().map(std::convert::Into::into).collect()}
            } else if is_option(ty) {
                quote::quote! {.map(std::convert::Into::into)}
            } else {
                quote::quote! { .into() }
            }
        };
        ConvertFieldMap::Suffix(suffix)
    }

    pub(crate) fn to_token(&self, ident: &TokenStream) -> TokenStream {
        match self {
            ConvertFieldMap::Map(map) => map.to_token_stream(),
            ConvertFieldMap::FieldFn(map) => quote! {
                #map(value.#ident)
            },
            ConvertFieldMap::StructFn(map) => quote! {
                #map(&value)
            },
            ConvertFieldMap::Suffix(suffix) => quote! {
                value.#ident #suffix
            },
        }
    }
}

#[derive(Clone)]
pub(crate) struct ConvertFieldArg {
    pub ignore: bool,
    pub map: ConvertFieldMap,
    pub rename: Option<LitStr>,
}

impl ConvertFieldArg {
    pub(crate) fn merge(&self, superiority: &Self) -> Self {
        Self {
            ignore: self.ignore || superiority.ignore,
            map: {
                match &superiority.map {
                    ConvertFieldMap::Suffix(_) => self.map.clone(),
                    _ => superiority.map.clone(),
                }
            },
            rename: match &superiority.rename {
                Some(rename) => Some(rename.clone()),
                None => self.rename.clone(),
            },
        }
    }
}

const NOT_FIRST: &str = "target attribute must be first";

impl ConvertFieldArg {
    pub(crate) fn from_meta_iter(
        ty: &Type,
        meta_iter: impl IntoIterator<Item = Meta>,
    ) -> syn::Result<Self> {
        let mut ignore = false;
        let mut map = None;
        let mut rename = None;

        macro_rules! check_duplicate_map {
            ($span:expr) => {
                check_duplicate!(
                    $span,
                    map,
                    "chose one of `map`, `map_field` or `map_struct`"
                );
            };
        }

        for meta in meta_iter {
            match meta {
                Meta::Path(path) if path.is_ident("ignore") => {
                    check_duplicate!(path.span(), ignore, ignore);
                    ignore = true;
                }
                Meta::NameValue(meta) if meta.path.is_ident("map") => {
                    check_duplicate_map!(meta.path.span());
                    map = Some(ConvertFieldMap::Map(meta.value));
                }
                Meta::NameValue(meta) if meta.path.is_ident("map_field") => {
                    check_duplicate_map!(meta.path.span());
                    let Expr::Path(path) = meta.value else {
                        return Err(syn::Error::new(meta.value.span(), "expected path"));
                    };
                    map = Some(ConvertFieldMap::FieldFn(path));
                }
                Meta::NameValue(meta) if meta.path.is_ident("map_struct") => {
                    check_duplicate_map!(meta.path.span());
                    let Expr::Path(path) = meta.value else {
                        return Err(syn::Error::new(meta.value.span(), "expected path"));
                    };
                    map = Some(ConvertFieldMap::StructFn(path));
                }
                Meta::NameValue(meta) if meta.path.is_ident("rename") => {
                    check_duplicate!(meta.path.span(), rename);
                    let Expr::Lit(lit) = meta.value else {
                        return Err(syn::Error::new(meta.value.span(), "expected literal"));
                    };

                    let Lit::Str(lit_str) = lit.lit else {
                        return Err(syn::Error::new_spanned(lit, "expected string literal"));
                    };

                    rename = Some(lit_str);
                }
                Meta::List(list) if list.path.is_ident("from") => {
                    return Err(syn::Error::new(list.span(), NOT_FIRST))
                }
                Meta::List(list) if list.path.is_ident("into") => {
                    return Err(syn::Error::new(list.span(), NOT_FIRST))
                }
                Meta::List(list) if list.path.is_ident("from_into") => {
                    return Err(syn::Error::new(list.span(), NOT_FIRST))
                }
                _ => {
                    return Err(syn::Error::new(
                        meta.span(),
                        "unrecognized convert attribute",
                    ))
                }
            }
        }

        Ok(Self {
            ignore,
            map: map.unwrap_or_else(|| ConvertFieldMap::gen_suffix(ty)),
            rename,
        })
    }
}
