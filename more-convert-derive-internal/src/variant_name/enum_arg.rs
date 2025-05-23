use convert_case::Case;
use syn::{spanned::Spanned, Meta};

use crate::{check_duplicate, from_str_to_case, parse_meta_attrs, require_lit_str};

pub(crate) struct VariantNameEnumArg {
    pub without_trait: bool,
    pub rename_all: Option<Case>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

impl VariantNameEnumArg {
    pub(crate) fn from_derive(derive: &syn::DeriveInput) -> syn::Result<Self> {
        let mut rename_all: Option<Case> = None;
        let mut prefix: Option<String> = None;
        let mut suffix: Option<String> = None;
        let mut without_trait = false;

        parse_meta_attrs("variant_name", &derive.attrs, |meta| {
            match meta {
                Meta::NameValue(meta) if meta.path.is_ident("rename_all") => {
                    check_duplicate!(meta.span(), rename_all);

                    let string = require_lit_str(&meta, &meta.value)?;

                    rename_all = Some(
                        from_str_to_case(&string)
                            .ok_or_else(|| syn::Error::new(meta.span(), "invalid rename_all"))?,
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
                Meta::Path(meta) if meta.is_ident("without_trait") => {
                    check_duplicate!(meta.span(), without_trait, without_trait);

                    without_trait = true;
                }
                _ => return Err(syn::Error::new(meta.span(), "unexpected attribute")),
            }
            Ok(())
        })?;

        Ok(VariantNameEnumArg {
            without_trait,
            rename_all,
            prefix,
            suffix,
        })
    }
}
