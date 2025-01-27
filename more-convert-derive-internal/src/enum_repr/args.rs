use quote::ToTokens;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Ident, LitInt, Path, Variant};

pub(crate) struct EnumReprField {
    pub ident: Ident,
    pub discriminant: u16,
}

impl EnumReprField {
    pub(crate) fn from_variants(variants: &Punctuated<Variant, Comma>) -> syn::Result<Vec<Self>> {
        let mut fields = Vec::with_capacity(variants.len());
        let mut iter = variants.iter();

        let variant = iter
            .next()
            .ok_or_else(|| syn::Error::new(variants.span(), "expected at least one variant"))?;

        let mut prev_discriminant = match variant.discriminant.as_ref() {
            Some((_, expr)) => {
                syn::parse2::<LitInt>(expr.to_token_stream())?.base10_parse::<u16>()?
            }
            None => 0,
        };

        fields.push(EnumReprField {
            ident: variant.ident.clone(),
            discriminant: prev_discriminant,
        });

        for variant in iter {
            let discriminant = match variant.discriminant.as_ref() {
                Some((_, expr)) => {
                    syn::parse2::<LitInt>(expr.to_token_stream())?.base10_parse::<u16>()?
                }
                None => prev_discriminant + 1,
            };
            prev_discriminant = discriminant;
            fields.push(EnumReprField {
                ident: variant.ident.clone(),
                discriminant,
            });
        }

        Ok(fields)
    }
}

#[derive(Default)]
pub(crate) struct EnumReprOption {
    pub serde: bool,
}

impl EnumReprOption {
    pub(crate) fn from_attr(attr: &syn::Attribute) -> syn::Result<Self> {
        let mut option = EnumReprOption::default();
        let metas = attr.parse_args_with(Punctuated::<Path, Comma>::parse_terminated)?;
        for meta in metas {
            match meta.get_ident() {
                Some(ident) if ident == "serde" => {
                    option.serde = true;
                }
                _ => {
                    return Err(syn::Error::new(
                        meta.span(),
                        "unrecognized enum repr option",
                    ));
                }
            }
        }
        Ok(option)
    }
}
