use crate::require_enum;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Ident, LitInt};

struct EnumReprField<'a> {
    pub ident: &'a Ident,
    pub discriminant: u16,
}

pub fn derive_enum_repr(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let variants = require_enum(&input)?;
    for attr in &input.attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }

        match &attr.meta {
            syn::Meta::List(meta) => {
                let mut fields = Vec::with_capacity(variants.len());
                let mut iter = variants.iter();

                let variant = iter.next().ok_or_else(|| {
                    syn::Error::new(input.span(), "expected at least one variant")
                })?;

                let mut prev_discriminant = match variant.discriminant.as_ref() {
                    Some((_, expr)) => {
                        syn::parse2::<LitInt>(expr.to_token_stream())?.base10_parse::<u16>()?
                    }
                    None => 0,
                };

                fields.push(EnumReprField {
                    ident: &variant.ident,
                    discriminant: prev_discriminant,
                });

                for variant in variants {
                    let discriminant = match variant.discriminant.as_ref() {
                        Some((_, expr)) => {
                            syn::parse2::<LitInt>(expr.to_token_stream())?.base10_parse::<u16>()?
                        }
                        None => prev_discriminant + 1,
                    };
                    prev_discriminant = discriminant;
                    fields.push(EnumReprField {
                        ident: &variant.ident,
                        discriminant,
                    });
                }

                return derive_enum_repr_internal(&input, fields, &meta.tokens);
            }
            _ => {
                return Err(syn::Error::new(attr.span(), "expected `repr` attribute"));
            }
        }
    }
    Err(syn::Error::new(input.span(), "expected `repr` attribute"))
}

fn derive_enum_repr_internal(
    input: &syn::DeriveInput,
    fields: Vec<EnumReprField<'_>>,
    repr: &TokenStream,
) -> syn::Result<TokenStream> {
    let _ = fields;
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let token = quote::quote! {
        impl #impl_generics From<#ident> for #repr #ty_generics #where_clause {
            fn from(value: #ident) -> Self {
                value as #repr
            }
        }
    };
    Ok(token)
}
