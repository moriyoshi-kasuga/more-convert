use enum_arg::EnumNameEnumArg;
use proc_macro2::TokenStream;
use variant_arg::EnumNameVariantArg;

use crate::require_enum;

mod enum_arg;
mod variant_arg;

pub fn derive_enum_name(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let variants = require_enum(&input)?;
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let enum_arg = EnumNameEnumArg::from_derive(&input)?;

    let variant_name = variants.iter().map(|f| &f.ident).collect::<Vec<_>>();

    let mut body: Vec<TokenStream> = Vec::with_capacity(variants.len());

    for variant in variants {
        let variant_arg = EnumNameVariantArg::from_variant(variant, &enum_arg)?;
        body.push(variant_arg.into_token());
    }

    let token = quote::quote! {
        impl #impl_generics more_convert::EnumName for #ident #ty_generics #where_clause {
            fn enum_name(&self) -> &'static str {
                match self {
                    #(
                        #ident::#variant_name { .. } => #body,
                    )*
                }
            }
        }
    };
    Ok(token)
}
