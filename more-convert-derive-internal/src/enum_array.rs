use proc_macro2::TokenStream;

use crate::require_enum;

pub fn derive_enum_array(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let variants = require_enum(&input)?;
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let variants = variants.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let count = variants.len();

    let token = quote::quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub const COUNT: usize = #count;
            pub const VARIANTS: &[Self] = &[#(Self::#variants),*];
        }
    };

    Ok(token)
}
