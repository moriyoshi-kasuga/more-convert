use syn::{spanned::Spanned, Expr, Ident, Variant};

use crate::parse_nested_attrs;

pub(crate) struct EnumReprVariant<'a> {
    pub ident: &'a Ident,
    pub discriminant: Option<&'a Expr>,
    pub is_default: bool,
}

impl<'a> EnumReprVariant<'a> {
    pub(crate) fn from_variant(variant: &'a Variant) -> syn::Result<Self> {
        let mut is_default = false;

        parse_nested_attrs("enum_repr", &variant.attrs, |meta| {
            if meta.path.is_ident("default") {
                if is_default {
                    return Err(syn::Error::new(
                        meta.path.span(),
                        "duplicate `default` attribute",
                    ));
                }
                is_default = true;
                return Ok(());
            }
            Err(syn::Error::new(meta.path.span(), "unexpected attribute"))
        })?;

        Ok(Self {
            ident: &variant.ident,
            discriminant: variant.discriminant.as_ref().map(|(_, expr)| expr),
            is_default,
        })
    }
}
