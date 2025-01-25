use proc_macro2::TokenStream;
use syn::Ident;

use super::args::ConvertFieldArgs;

pub(crate) fn gen_into(
    into_ident: Ident,
    input: &syn::DeriveInput,
    fields: Vec<ConvertFieldArgs>,
) -> syn::Result<TokenStream> {
    todo!()
}
