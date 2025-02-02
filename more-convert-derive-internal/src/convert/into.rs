use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

use super::args::ConvertFieldArgs;

pub(crate) fn process_into(
    fields: &Vec<ConvertFieldArgs>,
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let result: (Vec<_>, Vec<_>) = fields
        .iter()
        .filter(|v| !v.ignore)
        .map(|v| {
            let token = v.map.to_token(&v.ident.to_token_stream());
            let ident = match &v.rename {
                Some(v) => Ident::new(&v.value(), v.span()).into_token_stream(),
                None => v.ident.to_token_stream(),
            };
            (ident, token)
        })
        .unzip();

    Ok(result)
}
