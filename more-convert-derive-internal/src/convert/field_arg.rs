use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, Expr, ExprPath, Field, Ident, Lit, LitStr, Meta,
    Token, Type,
};

use crate::{check_duplicate, is_option, is_vec};

use super::struct_arg::ConvertTarget;

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

#[derive(PartialEq)]
pub(crate) enum ConvertFieldArgKind {
    From(Ident),
    Into(Ident),
    All,
}

pub(crate) struct ConvertFieldArg {
    pub kind: ConvertFieldArgKind,
    pub ignore: bool,
    pub map: ConvertFieldMap,
    pub rename: Option<LitStr>,
}

type Args = Punctuated<Meta, Token![,]>;

impl ConvertFieldArg {
    pub(crate) fn from_meta(
        ty: &Type,
        kind: ConvertFieldArgKind,
        meta_iter: impl IntoIterator<Item = Meta>,
    ) -> syn::Result<Vec<Self>> {
        let mut vec = Vec::new();

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

        macro_rules! check_nest {
            ($current_kind:ident,$name:literal,$list:expr,$($kind:ident),*) => {
                if $current_kind != ConvertFieldArgKind::All {
                    return Err(syn::Error::new(
                        $list.span(),
                        concat!("not allowed nested `", $name, "`"),
                    ));
                }
                let mut meta = $list.parse_args_with(Args::parse_terminated)?.into_iter();
                let ident = meta
                    .next()
                    .ok_or_else(|| syn::Error::new($list.span(), "expected `ident`"))?;
                let ident = ident.require_path_only()?;
                let ident = ident
                    .get_ident()
                    .ok_or_else(|| syn::Error::new(ident.span(), "expected ident"))?;
                $(vec.extend(ConvertFieldArg::from_meta(
                    ty,
                    ConvertFieldArgKind::$kind(ident.clone()),
                    meta.clone(),
                )?);)*
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
                    check_nest!(kind, "from", list, From);
                }
                Meta::List(list) if list.path.is_ident("into") => {
                    check_nest!(kind, "into", list, Into);
                }
                Meta::List(list) if list.path.is_ident("from_into") => {
                    check_nest!(kind, "from_into", list, From, Into);
                }
                _ => {
                    return Err(syn::Error::new(
                        meta.span(),
                        "unrecognized convert attribute",
                    ))
                }
            }
        }

        vec.push(Self {
            kind,
            ignore,
            map: map.unwrap_or_else(|| ConvertFieldMap::Suffix(gen_suffix(ty))),
            rename,
        });

        Ok(vec)
    }
}

pub(crate) struct ConvertFieldArgs<'a> {
    pub ident: &'a Ident,
    pub arg: Vec<ConvertFieldArg>,
}

impl<'a> ConvertFieldArgs<'a> {
    pub(crate) fn get_top_priority_arg(&self, to: &ConvertTarget) -> &ConvertFieldArg {
        match to {
            ConvertTarget::From(ident) => {
                if let Some(from) = self.arg.iter().find(
                    |v| matches!(v.kind, ConvertFieldArgKind::From(ref kind_ident) if kind_ident == ident),
                ) {
                    return from;
                }
            }
            ConvertTarget::Into(ident) => {
                if let Some(into) = self.arg.iter().find(
                    |v| matches!(v.kind, ConvertFieldArgKind::Into(ref kind_ident) if kind_ident == ident)) {
                    return into;
                }
            }
        }
        // If there is no match, return the first All arg
        #[allow(clippy::unwrap_used)]
        self.arg
            .iter()
            .find(|v| matches!(v.kind, ConvertFieldArgKind::All))
            .unwrap()
    }

    pub(crate) fn from_field(field: &'a Field) -> syn::Result<Self> {
        let Some(ref ident) = field.ident else {
            return Err(syn::Error::new(field.span(), "expected named field"));
        };

        let mut arg = Vec::new();

        for attr in &field.attrs {
            if !attr.path().is_ident("convert") {
                continue;
            }

            let nested = attr.parse_args_with(Args::parse_terminated)?;
            arg.extend(ConvertFieldArg::from_meta(
                &field.ty,
                ConvertFieldArgKind::All,
                nested,
            )?);
        }

        if arg.is_empty() {
            arg.push(ConvertFieldArg {
                kind: ConvertFieldArgKind::All,
                ignore: false,
                map: ConvertFieldMap::Suffix(gen_suffix(&field.ty)),
                rename: None,
            });
        }

        Ok(ConvertFieldArgs { ident, arg })
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
