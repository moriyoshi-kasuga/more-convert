use crate::require_enum;
use enum_arg::EnumReprArg;
use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Expr, Ident};
use variant_arg::EnumReprVariant;

mod enum_arg;
mod internal;
mod variant_arg;

pub(crate) struct FinalVariantData<'a> {
    pub ident: &'a Ident,
    pub discriminant: Expr,
}

pub fn derive_enum_repr(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let variants = require_enum(&input)?;
    let mut repr: Option<TokenStream> = None;
    let mut enum_arg: Option<EnumReprArg> = None;
    for attr in &input.attrs {
        match attr.path().get_ident() {
            Some(ident) if ident == "repr" => {
                repr = Some(attr.parse_args()?);
            }
            Some(ident) if ident == "enum_repr" => {
                enum_arg = Some(attr.parse_args()?);
            }
            _ => {}
        }
    }

    let repr = repr.ok_or_else(|| syn::Error::new(input.span(), "expected `repr` attribute"))?;
    let option = enum_arg.unwrap_or_default();

    let variants_data = variants
        .iter()
        .map(EnumReprVariant::from_variant)
        .collect::<syn::Result<Vec<_>>>()?;

    let mut default = None;
    for v in &variants_data {
        if v.is_default {
            if default.is_some() {
                return Err(syn::Error::new(
                    v.ident.span(),
                    "duplicate `default` attribute",
                ));
            }
            default = Some(v.ident);
        }
    }

    let mut final_variants = Vec::with_capacity(variants.len());
    let mut prev_discriminant: Option<Expr> = None;

    for variant in &variants_data {
        let discriminant = match variant.discriminant {
            Some(expr) => expr.clone(),
            None => {
                if !option.implicit {
                    return Err(syn::Error::new(
                        variant.ident.span(),
                        "expected explicit discriminant (add #[enum_repr(implicit)] to enum attribute if you want it implicit)",
                    ));
                }
                match prev_discriminant {
                    Some(prev) => syn::parse_quote! { #prev + 1 },
                    None => syn::parse_quote! { 0 },
                }
            }
        };
        prev_discriminant = Some(discriminant.clone());
        final_variants.push(FinalVariantData {
            ident: variant.ident,
            discriminant,
        });
    }

    internal::derive_enum_repr_internal(&input, option, default, final_variants, repr)
}
