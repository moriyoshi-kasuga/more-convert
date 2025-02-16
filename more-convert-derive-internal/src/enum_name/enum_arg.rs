use std::str::FromStr;

use ident_case::RenameRule;
use syn::{spanned::Spanned, Meta};

use crate::{check_duplicate, parse_meta_attrs, require_lit_str};

pub(crate) struct EnumNameEnumArg {
    pub rename_all: RenameRule,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

impl EnumNameEnumArg {
    pub(crate) fn from_derive(derive: &syn::DeriveInput) -> syn::Result<Self> {
        let mut rename_all: Option<RenameRule> = None;
        let mut prefix: Option<String> = None;
        let mut suffix: Option<String> = None;

        parse_meta_attrs("enum_name", &derive.attrs, |meta| {
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
                _ => return Err(syn::Error::new(meta.span(), "unexpected attribute")),
            }
            Ok(())
        })?;

        Ok(EnumNameEnumArg {
            rename_all: rename_all.unwrap_or(RenameRule::None),
            prefix,
            suffix,
        })
    }
}
