use quote::ToTokens;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Ident, LitInt, Path, Variant};

pub(crate) struct EnumReprField {
    pub ident: Ident,
    pub discriminant: LitInt,
}

impl EnumReprField {
    pub(crate) fn from_variants(
        option: &EnumReprOption,
        variants: &Punctuated<Variant, Comma>,
    ) -> syn::Result<Vec<Self>> {
        let mut fields = Vec::with_capacity(variants.len());
        let mut iter = variants.iter();

        let variant = iter
            .next()
            .ok_or_else(|| syn::Error::new(variants.span(), "expected at least one variant"))?;

        let mut prev_discriminant = match variant.discriminant.as_ref() {
            Some((_, expr)) => syn::parse2::<LitInt>(expr.to_token_stream())?,
            None => {
                if !option.implicit {
                    return Err(syn::Error::new(
                        variant.span(),
                        concat!(
                            "expected explicit ",
                            "(add #[enum_repr(implicit)] to enum attribute ",
                            "if you want it implicit)",
                        ),
                    ));
                }
                LitInt::new("0", variant.span())
            }
        };

        fields.push(EnumReprField {
            ident: variant.ident.clone(),
            discriminant: prev_discriminant.clone(),
        });

        for variant in iter {
            let discriminant = match variant.discriminant.as_ref() {
                Some((_, expr)) => syn::parse2::<LitInt>(expr.to_token_stream())?,
                None => {
                    if !option.implicit {
                        return Err(syn::Error::new(
                            variant.span(),
                            concat!(
                                "expected explicit ",
                                "(add #[enum_repr(implicit)] to enum attribute ",
                                "if you want it implicit)",
                            ),
                        ));
                    }
                    LitInt::new(
                        &(prev_discriminant
                            .base10_digits()
                            .parse::<u128>()
                            .map_err(|err| syn::Error::new(variant.span(), err))?
                            + 1)
                        .to_string(),
                        variant.span(),
                    )
                }
            };
            prev_discriminant = discriminant.clone();
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
    pub implicit: bool,
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
                Some(ident) if ident == "implicit" => {
                    option.implicit = true;
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
