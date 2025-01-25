use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::Ident;

use super::args::ConvertFieldArgs;

pub(crate) fn gen_into(
    into_ident: Ident,
    input: &syn::DeriveInput,
    fields: Vec<ConvertFieldArgs>,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (idents, tokens): (Vec<_>, Vec<_>) = fields
        .into_iter()
        .filter(|v| !v.ignore)
        .map(|v| {
            let token = v.map.into_token(&v.ident.to_token_stream());
            let ident = match v.rename {
                Some(v) => Ident::new(&v.value(), v.span()).into_token_stream(),
                None => v.ident.to_token_stream(),
            };
            (ident, token)
        })
        .unzip();

    let token = quote! {
        impl #impl_generics std::convert::From<#ident> for #into_ident #ty_generics #where_clause {
            fn from(value: #ident) -> Self {
                Self {
                    #(
                        #idents: #tokens,
                    )*
                }
            }
        }
    };
    Ok(token)
}
