use proc_macro2::TokenStream;
use quote::format_ident;

use super::{variant_arg::EnumReprDefault, EnumReprArg, EnumReprVariant};

pub(crate) fn derive_enum_repr_internal(
    input: &syn::DeriveInput,
    enum_arg: EnumReprArg,
    default: Option<EnumReprDefault>,
    fields: Vec<EnumReprVariant>,
    repr: TokenStream,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (discriminant_idents, (idents, discriminants)): (Vec<_>, (Vec<_>, Vec<_>)) = fields
        .iter()
        .map(|v| {
            (
                quote::format_ident!("{}_{}", ident, &v.ident),
                (v.ident, v.discriminant.as_ref()),
            )
        })
        .unzip();

    let to_repr = match default {
        Some(EnumReprDefault(default_ident)) => quote::quote! {
            impl #impl_generics From<#repr> for #ident #ty_generics #where_clause {
                fn from(value: #repr) -> Self {
                    match value {
                        #(#discriminant_idents => Self::#idents,)*
                        _ => Self::#default_ident,
                    }
                }
            }
        },
        None => quote::quote! {
            impl #impl_generics TryFrom<#repr> for #ident #ty_generics #where_clause {
                type Error = String;

                fn try_from(value: #repr) -> Result<Self, Self::Error> {
                    Ok(match value {
                        #(#discriminant_idents => Self::#idents,)*
                        _ => return Err(format!(concat!("invalid ", stringify!(#ident), ": {}"), value)),
                    })
                }
            }
        },
    };

    let mut token = quote::quote! {
        #[allow(non_upper_case_globals)]
        const _: () = {
            #(
                const #discriminant_idents: #repr = #discriminants;
            )*

            impl #impl_generics From<#ident> for #repr #ty_generics #where_clause {
                fn from(value: #ident) -> Self {
                    value as #repr
                }
            }

            #to_repr
        };
    };

    if enum_arg.serde {
        let impl_deserialize = match default {
            Some(EnumReprDefault(_)) => quote::quote! {
                impl<'de> serde::Deserialize<'de> for #ident {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        let deserialized = #repr::deserialize(deserializer)?;
                        Ok(deserialized.into())
                    }
                }
            },
            None => quote::quote! {
                impl<'de> serde::Deserialize<'de> for #ident {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        let deserialized = #repr::deserialize(deserializer)?;
                        deserialized.try_into().map_err(serde::de::Error::custom)
                    }
                }
            },
        };

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

            #impl_deserialize
        });
    };

    Ok(token)
}
