mod args;

use crate::require_enum;
use args::{EnumReprField, EnumReprOption};
use proc_macro2::TokenStream;
use quote::format_ident;
use syn::spanned::Spanned;

pub fn derive_enum_repr(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let variants = require_enum(&input)?;
    let mut repr: Option<(TokenStream, Vec<EnumReprField>)> = None;
    let mut option: Option<EnumReprOption> = None;
    for attr in &input.attrs {
        match attr.path().get_ident() {
            Some(ident) if ident == "repr" => {
                repr = Some((attr.parse_args()?, EnumReprField::from_variants(variants)?));
            }
            Some(ident) if ident == "enum_repr" => {
                option = Some(EnumReprOption::from_attr(attr)?);
            }
            _ => {}
        }
    }
    match repr {
        Some((repr, fields)) => {
            derive_enum_repr_internal(&input, option.unwrap_or_default(), fields, repr)
        }
        None => Err(syn::Error::new(input.span(), "expected `repr` attribute")),
    }
}

fn derive_enum_repr_internal(
    input: &syn::DeriveInput,
    option: EnumReprOption,
    fields: Vec<EnumReprField>,
    repr: TokenStream,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (idents, discriminants): (Vec<_>, Vec<_>) = fields
        .into_iter()
        .map(|v| (v.ident, v.discriminant))
        .unzip();

    let mut token = quote::quote! {
        impl #impl_generics From<#ident> for #repr #ty_generics #where_clause {
            fn from(value: #ident) -> Self {
                value as #repr
            }
        }

        impl #impl_generics TryFrom<#repr> for #ident #ty_generics #where_clause {
            type Error = String;

            fn try_from(value: #repr) -> Result<Self, Self::Error> {
                Ok(match value {
                    #(#discriminants => Self::#idents,)*
                    _ => return Err(format!(concat!("invalid ", stringify!(#ident), ": {}"), value)),
                })
            }
        }
    };

    if option.serde {
        let serde = format_ident!("serialize_{}", repr.to_string());
        token.extend(quote::quote! {
            impl serde::Serialize for #ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    serializer.#serde(*self as #repr)
                }
            }

            impl<'de> serde::Deserialize<'de> for #ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let deserialized = #repr::deserialize(deserializer)?;
                    deserialized.try_into().map_err(serde::de::Error::custom)
                }
            }
        });
    };

    Ok(token)
}
