use args::{ConvertArgs, ConvertFieldArgs};
use proc_macro2::TokenStream;
use syn::spanned::Spanned;

use crate::require_named_field_struct;

mod args;
mod from;
mod into;

pub fn derive_convert(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let fields = require_named_field_struct(&input)?;
    let attr = 'bar: {
        for attr in &input.attrs {
            if attr.path().is_ident("convert") {
                break 'bar attr;
            }
        }
        return Err(syn::Error::new(
            input.span(),
            "expected `convert` attribute",
        ));
    };

    let struct_args = ConvertArgs::from_attr(attr)?;

    let fields = fields
        .named
        .iter()
        .map(ConvertFieldArgs::from_field)
        .collect::<syn::Result<Vec<_>>>()?;

    match struct_args {
        ConvertArgs::From(ident) => from::gen_from(&input, ident, fields),
        ConvertArgs::Into(ident) => into::gen_into(ident, &input, fields),
    }
}
