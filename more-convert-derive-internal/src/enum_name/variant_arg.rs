use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Ident, Meta, Type};

use crate::{check_duplicate, parse_meta_attrs, require_lit_str, unraw};

use super::enum_arg::EnumNameEnumArg;

pub(crate) struct EnumNameVariantArg {
    pub name: String,
    pub nest: Option<(Option<Ident>, Type)>,
}

impl EnumNameVariantArg {
    pub(crate) fn into_token(self) -> (TokenStream, TokenStream) {
        if let Some((ident, ty)) = self.nest {
            let matches = match ident {
                Some(ident) => quote::quote! { { #ident: _nested_enum_name, .. } },
                None => quote::quote! { (_nested_enum_name) },
            };
            let expr = quote::quote! {
                <#ty as ::more_convert::EnumName>::enum_name(_nested_enum_name)
            };

            (matches, expr)
        } else {
            let matches = quote::quote! { { .. } };

            let expr = self.name.into_token_stream();

            (matches, expr)
        }
    }

    pub(crate) fn from_variant(
        variant: &syn::Variant,
        enum_arg: &EnumNameEnumArg,
    ) -> syn::Result<Self> {
        let mut rename: Option<String> = None;
        let mut nest: Option<(Option<Ident>, Type)> = None;

        parse_meta_attrs("enum_name", &variant.attrs, |meta| {
            match meta {
                Meta::NameValue(meta) if meta.path.is_ident("rename") => {
                    check_duplicate!(meta.span(), rename);
                    let string = require_lit_str(&meta, &meta.value)?;
                    rename = Some(string);
                }
                Meta::Path(meta) if meta.is_ident("nest") => {
                    check_duplicate!(meta.span(), nest);
                    let fields = match &variant.fields {
                        syn::Fields::Named(fields_named) => &fields_named.named,
                        syn::Fields::Unnamed(fields_unnamed) => &fields_unnamed.unnamed,
                        syn::Fields::Unit => {
                            return Err(syn::Error::new_spanned(
                                meta,
                                "unit variant cannot be nested",
                            ))
                        }
                    };
                    let Some(field) = fields.first() else {
                        return Err(syn::Error::new_spanned(meta, "variant has no fields"));
                    };
                    if fields.len() > 1 {
                        return Err(syn::Error::new_spanned(meta, "variant has multiple fields"));
                    }

                    nest = Some((field.ident.clone(), field.ty.clone()));
                }
                _ => return Err(syn::Error::new_spanned(meta, "unexpected attribute")),
            }
            Ok(())
        })?;

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

        Ok(Self { name, nest })
    }
}
