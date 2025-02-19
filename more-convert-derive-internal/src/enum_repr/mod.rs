use crate::require_enum;
use enum_arg::EnumReprArg;
use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use variant_arg::EnumReprVariant;

mod enum_arg;
mod internal;
mod variant_arg;

pub fn derive_enum_repr(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let variants = require_enum(&input)?;
    let mut repr: Option<TokenStream> = None;
    let mut enum_arg: Option<EnumReprArg> = None;
    for attr in &input.attrs {
        match attr.path().get_ident() {
            Some(ident) if ident == "repr" => {
                repr = Some(attr.parse_args()?);
            }
            Some(ident) if ident == "enum_repr" => {
                enum_arg = Some(EnumReprArg::from_attr(attr)?);
            }
            _ => {}
        }
    }
    match repr {
        Some(repr) => {
            let option = enum_arg.unwrap_or_default();
            let (default, fields) = EnumReprVariant::from_variants(&option, variants)?;
            internal::derive_enum_repr_internal(&input, option, default, fields, repr)
        }
        None => Err(syn::Error::new(input.span(), "expected `repr` attribute")),
    }
}
