use syn::{spanned::Spanned, Meta};

use crate::parse_meta_attr;

#[derive(Default)]
pub(crate) struct EnumReprArg {
    pub serde: bool,
    pub implicit: bool,
}

impl EnumReprArg {
    pub(crate) fn from_attr(attr: &syn::Attribute) -> syn::Result<Self> {
        let mut option = EnumReprArg::default();
        parse_meta_attr(attr, |meta| {
            match meta {
                Meta::Path(path) if path.is_ident("serde") => {
                    option.serde = true;
                }
                Meta::Path(path) if path.is_ident("implicit") => {
                    option.implicit = true;
                }
                _ => {
                    return Err(syn::Error::new(meta.span(), "unexpected attribute"));
                }
            };
            Ok(())
        })?;

        Ok(option)
    }
}
