use std::str::FromStr;

use ident_case::RenameRule;
use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, spanned::Spanned, Meta, Token};

use crate::{check_duplicate, require_lit_str, unraw};

pub(crate) struct EnumNameFieldArg {
    pub name: String,
}

impl EnumNameFieldArg {
    pub(crate) fn into_token(self) -> TokenStream {
        let name = self.name;

        quote::quote! {
            #name
        }
    }
}

pub(crate) struct EnumNameEnumArg {
    pub rename_all: RenameRule,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

pub(crate) fn variant_attr(
    variant: &syn::Variant,
    enum_arg: &EnumNameEnumArg,
) -> syn::Result<EnumNameFieldArg> {
    let mut rename: Option<String> = None;

    for attr in &variant.attrs {
        if !attr.path().is_ident("enum_name") {
            continue;
        }

        let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

        for meta in nested {
            match meta {
                Meta::NameValue(meta) if meta.path.is_ident("rename") => {
                    check_duplicate!(meta.span(), rename);
                    let string = require_lit_str(&meta, &meta.value)?;
                    rename = Some(string);
                }
                _ => return Err(syn::Error::new_spanned(meta, "unexpected attribute")),
            }
        }
    }

    let name = match rename {
        Some(x) => x,
        None => {
            let mut name = unraw(&variant.ident);
            name = enum_arg.rename_all.apply_to_variant(name);
            if let Some(prefix) = &enum_arg.prefix {
                name = format!("{prefix}{name}");
            };
            if let Some(suffix) = &enum_arg.suffix {
                name = format!("{name}{suffix}");
            };
            name
        }
    };
    Ok(EnumNameFieldArg { name })
}

pub(crate) fn enum_attr(derive: &syn::DeriveInput) -> syn::Result<EnumNameEnumArg> {
    let mut rename_all: Option<RenameRule> = None;
    let mut prefix: Option<String> = None;
    let mut suffix: Option<String> = None;

    for attr in &derive.attrs {
        if !attr.path().is_ident("enum_name") {
            continue;
        }

        let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

        for meta in nested {
            match meta {
                Meta::NameValue(meta) if meta.path.is_ident("rename_all") => {
                    check_duplicate!(meta.span(), rename_all);

                    let string = require_lit_str(&meta, &meta.value)?;

                    rename_all = Some(
                        RenameRule::from_str(&string)
                            .map_err(|_| syn::Error::new(meta.span(), "invalid RenameRule"))?,
                    );
                }
                Meta::NameValue(meta) if meta.path.is_ident("prefix") => {
                    check_duplicate!(meta.span(), prefix);

                    let string = require_lit_str(&meta, &meta.value)?;

                    prefix = Some(string);
                }
                Meta::NameValue(meta) if meta.path.is_ident("suffix") => {
                    check_duplicate!(meta.span(), suffix);

                    let string = require_lit_str(&meta, &meta.value)?;

                    suffix = Some(string);
                }
                _ => return Err(syn::Error::new_spanned(meta, "unexpected attribute")),
            }
        }
    }
    Ok(EnumNameEnumArg {
        rename_all: rename_all.unwrap_or(RenameRule::None),
        prefix,
        suffix,
    })
}
