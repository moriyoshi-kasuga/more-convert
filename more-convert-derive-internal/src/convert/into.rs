use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;

use super::{field_arg::ConvertFieldArgs, ConvertTarget};

pub(crate) fn process_into(
    ident: &Ident,
    fields: &Vec<ConvertFieldArgs>,
) -> syn::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let into = ConvertTarget::Into(ident.clone());
    let mut result: (Vec<TokenStream>, Vec<TokenStream>) = Default::default();
    for v in fields {
        let arg = v.get_top_priority_arg(&into);
        if arg.ignore {
            continue;
        }
        let token = arg.map.to_token(&v.ident.to_token_stream());
        let ident = match &arg.rename {
            Some(v) => Ident::new(&v.value(), v.span()).into_token_stream(),
            None => v.ident.to_token_stream(),
        };

        result.0.push(ident);
        result.1.push(token);
    }

    Ok(result)
}
