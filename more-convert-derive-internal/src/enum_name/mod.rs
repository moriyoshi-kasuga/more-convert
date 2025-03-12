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

    let mut matches: Vec<TokenStream> = Vec::with_capacity(variants.len());
    let mut body: Vec<TokenStream> = Vec::with_capacity(variants.len());

    for variant in variants {
        let variant_arg = EnumNameVariantArg::from_variant(variant, &enum_arg)?;
        let (matches_token, variant_arg) = variant_arg.into_token();
        matches.push(matches_token);
        body.push(variant_arg);
    }

    let impl_token = if enum_arg.without_trait {
        quote::quote! {
            #ident
        }
    } else {
        quote::quote! {
            more_convert::EnumName for #ident
        }
    };

    let fn_token = if enum_arg.without_trait {
        quote::quote! { const fn }
    } else {
        quote::quote! { fn }
    };

    let token = quote::quote! {
        impl #impl_generics #impl_token #ty_generics #where_clause {
            #fn_token enum_name(&self) -> &'static str {
                match self {
                    #(
                        #ident::#variant_name #matches => #body,
                    )*
                }
            }
        }
    };
    Ok(token)
}
