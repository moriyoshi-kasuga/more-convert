use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Meta};

use crate::{check_duplicate, parse_meta_attrs, require_lit_str, unraw};

use super::enum_arg::EnumNameEnumArg;

pub(crate) struct EnumNameVariantArg {
    pub name: String,
}

impl EnumNameVariantArg {
    pub(crate) fn into_token(self) -> TokenStream {
        let name = self.name;

        quote::quote! {
            #name
        }
    }

    pub(crate) fn from_variant(
        variant: &syn::Variant,
        enum_arg: &EnumNameEnumArg,
    ) -> syn::Result<Self> {
        let mut rename: Option<String> = None;

        parse_meta_attrs("enum_name", &variant.attrs, |meta| {
            match meta {
                Meta::NameValue(meta) if meta.path.is_ident("rename") => {
                    check_duplicate!(meta.span(), rename);
                    let string = require_lit_str(&meta, &meta.value)?;
                    rename = Some(string);
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
        Ok(Self { name })
    }
}
