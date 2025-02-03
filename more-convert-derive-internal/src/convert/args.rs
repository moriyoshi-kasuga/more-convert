use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    braced, parenthesized, punctuated::Punctuated, spanned::Spanned, token::Paren, Expr, ExprPath,
    Field, Ident, Lit, LitStr, Meta, MetaList, Token, Type,
};

use crate::{is_option, is_vec};

pub(crate) enum ConvertArgs {
    From(Ident),
    Into(Ident),
}

impl ConvertArgs {
    pub(crate) fn from_attr(attr: &syn::Attribute) -> syn::Result<Vec<Self>> {
        let list: MetaList = attr.parse_args()?;

        let Some(ident) = list.path.get_ident() else {
            return Err(syn::Error::new(list.span(), "expected `from` or `into`"));
        };

        type Idents = syn::punctuated::Punctuated<syn::Ident, syn::Token![,]>;

        match ident.to_string().as_str() {
            "from" => {
                let idents: Idents = list.parse_args_with(Idents::parse_terminated)?;
                Ok(idents.into_iter().map(ConvertArgs::From).collect())
            }
            "into" => {
                let idents: Idents = list.parse_args_with(Idents::parse_terminated)?;
                Ok(idents.into_iter().map(ConvertArgs::Into).collect())
            }
            "from_into" => {
                let idents: Idents = list.parse_args_with(Idents::parse_terminated)?;
                let mut idents_vec = Vec::with_capacity(idents.len() * 2);
                for ident in idents {
                    idents_vec.push(ConvertArgs::From(ident.clone()));
                    idents_vec.push(ConvertArgs::Into(ident));
                }
                Ok(idents_vec)
            }
            _ => Err(syn::Error::new(ident.span(), "expected `from` or `into`")),
        }
    }
}

pub(crate) enum ConvertFieldMap {
    Map(Expr),
    FieldFn(ExprPath),
    StructFn(ExprPath),
    Suffix(TokenStream),
}

impl ConvertFieldMap {
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

pub(crate) enum ConvertFieldArgKind {
    From(Ident),
    Into(Ident),
    All,
}

pub(crate) struct ConvertFieldArg {
    pub kind: Vec<ConvertFieldArgKind>,
    pub ignore: bool,
    pub map: ConvertFieldMap,
    pub rename: Option<LitStr>,
}

pub(crate) struct ConvertFieldArgs<'a> {
    pub ident: &'a Ident,
    pub arg: ConvertFieldArg,
}

impl<'a> ConvertFieldArgs<'a> {
    pub(crate) fn from_field(field: &'a Field) -> syn::Result<Vec<Self>> {
        let Some(ref ident) = field.ident else {
            return Err(syn::Error::new(field.span(), "expected named field"));
        };

        macro_rules! check_duplicate {
            ($span:expr, $variant:ident) => {
                check_duplicate!(@__message $span, $variant, $variant.is_some(),);
            };
            ($span:expr, $variant:ident, $additional:literal) => {
                check_duplicate!(@__message $span, $variant, $variant.is_some(), $additional);
            };
            ($span:expr, $variant:ident, $expr:expr) => {
                check_duplicate!(@__message $span, $variant, $expr,);
            };
            (@__message $span:expr, $variant:ident, $expr:expr, $($additional:expr)?) => {
                check_duplicate!(@__final $span, $expr, concat!("duplicate `", stringify!($variant), "` attribute.", $(" ", $additional)?));
            };
            (@__final $span:expr, $expr:expr, $message:expr) => {
                if $expr {
                    return Err(syn::Error::new($span, $message));
                }
            };
        }

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

        for attr in &field.attrs {
            if !attr.path().is_ident("convert") {
                continue;
            }

            let token = attr.parse_args::<TokenStream>()?;

            let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
            for meta in nested {
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
                    _ => {
                        return Err(syn::Error::new(
                            meta.span(),
                            "unrecognized convert attribute",
                        ))
                    }
                }
            }
        }

        Ok(ConvertFieldArgs {
            ident,
            ignore,
            map: map.unwrap_or_else(|| ConvertFieldMap::Suffix(gen_suffix(&field.ty))),
            rename,
        })
    }
}

fn gen_suffix(ty: &Type) -> TokenStream {
    if is_vec(ty) {
        return quote::quote! {.into_iter().map(std::convert::Into::into).collect()};
    }
    if is_option(ty) {
        return quote::quote! {.map(std::convert::Into::into)};
    }
    quote::quote! { .into() }
}
