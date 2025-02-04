use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

use super::{args::ConvertFieldArgs, ConvertArgs};

pub(crate) fn process_from(
    ident: &Ident,
    fields: &Vec<ConvertFieldArgs>,
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let from = ConvertArgs::From(ident.clone());
    let mut result: (Vec<TokenStream>, Vec<TokenStream>) = Default::default();
    for v in fields {
        let arg = v.get_top_priority_arg(&from);
        let ident = match &arg.rename {
            Some(v) => Ident::new(&v.value(), v.span()).into_token_stream(),
            None => v.ident.to_token_stream(),
        };
        let token = arg.map.to_token(&ident);

        result.0.push(ident);
        result.1.push(token);
    }

    Ok(result)
}
