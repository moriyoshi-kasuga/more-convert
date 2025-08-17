use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Fields, Ident, Type};

use crate::{require_enum, unraw};

mod args;

use args::VariantNameArgs;

pub fn derive_variant_name(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let variants = require_enum(&input)?;
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut enum_args = VariantNameArgs::default();
    for attr in &input.attrs {
        if attr.path().is_ident("variant_name") {
            enum_args = attr.parse_args()?;
            break;
        }
    }

    let mut match_arms = Vec::with_capacity(variants.len());

    for variant in variants {
        let mut variant_args = VariantNameArgs::default();
        for attr in &variant.attrs {
            if attr.path().is_ident("variant_name") {
                variant_args = attr.parse_args()?;
                break;
            }
        }

        let variant_ident = &variant.ident;

        let (matches, body) = if variant_args.nest.is_some() {
            let (ident, ty) = get_nested_field(variant)?;
            let matches = match ident {
                Some(ident) => quote::quote! { { #ident: _nested_variant_name, .. } },
                None => quote::quote! { (_nested_variant_name) },
            };
            let expr = quote::quote! {
                <#ty as more_convert::VariantName>::variant_name(_nested_variant_name)
            };
            (matches, expr)
        } else {
            let name = match variant_args.rename {
                Some(lit) => lit.value(),
                None => {
                    let mut name = unraw(variant_ident);
                    if let Some(rename_all) = enum_args.rename_all {
                        name = name.to_case(rename_all);
                    }
                    if let Some(prefix) = &enum_args.prefix {
                        name = format!("{}{}", prefix.value(), name);
                    }
                    if let Some(suffix) = &enum_args.suffix {
                        name = format!("{}{}", name, suffix.value());
                    }
                    name
                }
            };
            (quote::quote! { { .. } }, name.into_token_stream())
        };

        match_arms.push(quote::quote! {
            Self::#variant_ident #matches => #body
        });
    }

    let impl_token = if enum_args.without_trait.is_some() {
        quote::quote! { #ident }
    } else {
        quote::quote! { more_convert::VariantName for #ident }
    };

    let fn_token = if enum_args.without_trait.is_some() {
        quote::quote! { const fn }
    } else {
        quote::quote! { fn }
    };

    let token = quote::quote! {
        impl #impl_generics #impl_token #ty_generics #where_clause {
            #fn_token variant_name(&self) -> &'static str {
                match self {
                    #(#match_arms,)*
                }
            }
        }
    };
    Ok(token)
}

fn get_nested_field(variant: &syn::Variant) -> syn::Result<(Option<&Ident>, &Type)> {
    let fields = match &variant.fields {
        Fields::Named(f) => &f.named,
        Fields::Unnamed(f) => &f.unnamed,
        Fields::Unit => {
            return Err(syn::Error::new_spanned(
                variant,
                "unit variant cannot be nested",
            ));
        }
    };

    let Some(field) = fields.first() else {
        return Err(syn::Error::new_spanned(variant, "variant has no fields"));
    };

    if fields.len() > 1 {
        return Err(syn::Error::new_spanned(
            variant,
            "variant has multiple fields",
        ));
    }

    Ok((field.ident.as_ref(), &field.ty))
}
