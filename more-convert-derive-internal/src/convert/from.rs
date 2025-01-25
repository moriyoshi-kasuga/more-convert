use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

use super::args::{ConvertFieldArgs, ConvertFieldMap};

pub(crate) fn gen_from(
    input: &syn::DeriveInput,
    from_ident: Ident,
    fields: Vec<ConvertFieldArgs>,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (idents, tokens): (Vec<_>, Vec<_>) = fields
        .into_iter()
        .filter(|v| !v.ignore)
        .map(|v| {
            let ident = match v.rename {
                Some(v) => Ident::new(&v.value(), v.span()).into_token_stream(),
                None => v.ident.to_token_stream(),
            };
            let token: TokenStream = {
                match v.map {
                    ConvertFieldMap::Map(map) => map.into_token_stream(),
                    ConvertFieldMap::FieldFn(map) => quote! {
                        #map(value.#ident)
                    },
                    ConvertFieldMap::StructFn(map) => quote! {
                        #map(&value)
                    },
                    ConvertFieldMap::Suffix(suffix) => quote! {
                        value.#ident #suffix
                    },
                }
            };
            (v.ident, token)
        })
        .unzip();

    let token = quote! {
        impl #impl_generics std::convert::From<#from_ident> for #ident #ty_generics #where_clause {
            fn from(value: #from_ident) -> Self {
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
