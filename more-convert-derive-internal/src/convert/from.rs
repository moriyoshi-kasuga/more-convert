use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

use super::args::ConvertFieldArgs;

pub(crate) fn process_from(
    fields: &Vec<ConvertFieldArgs>,
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let result: (Vec<_>, Vec<_>) = fields
        .iter()
        .filter(|v| !v.ignore)
        .map(|v| {
            let ident = match &v.rename {
                Some(v) => Ident::new(&v.value(), v.span()).into_token_stream(),
                None => v.ident.to_token_stream(),
            };
            let token = v.map.to_token(&ident);
            (v.ident.into_token_stream(), token)
        })
        .unzip();

    Ok(result)
}
