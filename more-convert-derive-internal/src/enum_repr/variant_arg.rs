use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Expr, Ident, Variant};

use crate::{check_duplicate, parse_nested_attrs, MaybeOwned};

use super::enum_arg::EnumReprArg;

pub(crate) struct EnumReprDefault<'a>(pub &'a Ident);

pub(crate) struct EnumReprVariant<'a> {
    pub ident: &'a Ident,
    pub discriminant: MaybeOwned<'a, Expr>,
}

const NO_VARIANTS: &str = "expected at least one variant";
const EXPLICIT: &str =
    "expected explicit (add #[enum_repr(implicit)] to enum attribute if you want it implicit)";

impl<'a> EnumReprVariant<'a> {
    pub(crate) fn from_variants(
        option: &EnumReprArg,
        variants: &'a Punctuated<Variant, Comma>,
    ) -> syn::Result<(Option<EnumReprDefault<'a>>, Vec<Self>)> {
        let mut default = None;

        for field in variants {
            parse_nested_attrs("enum_repr", &field.attrs, |meta| {
                check_duplicate!(meta.path.span(), default);
                if meta.path.is_ident("default") {
                    default = Some(EnumReprDefault(&field.ident));
                    return Ok(());
                }
                Err(syn::Error::new(meta.path.span(), "unexpected attribute"))
            })?;
        }

        let mut fields = Vec::with_capacity(variants.len());
        let mut iter = variants.iter();

        let variant = iter
            .next()
            .ok_or_else(|| syn::Error::new(variants.span(), NO_VARIANTS))?;

        let mut prev_discriminant = match variant.discriminant.as_ref() {
            Some((_, expr)) => MaybeOwned::Borrowed(expr),
            None => {
                if !option.implicit {
                    return Err(syn::Error::new(variant.span(), EXPLICIT));
                }
                MaybeOwned::Owned(syn::parse_quote! {
                    0
                })
            }
        };

        fields.push(EnumReprVariant {
            ident: &variant.ident,
            discriminant: prev_discriminant.clone(),
        });

        for variant in iter {
            let discriminant = match variant.discriminant.as_ref() {
                Some((_, expr)) => MaybeOwned::Borrowed(expr),
                None => {
                    if !option.implicit {
                        return Err(syn::Error::new(variant.span(), EXPLICIT));
                    }
                    let prev_discriminant = prev_discriminant.as_ref();
                    MaybeOwned::Owned(syn::parse_quote! {
                        #prev_discriminant + 1
                    })
                }
            };
            prev_discriminant = discriminant.clone();
            fields.push(EnumReprVariant {
                ident: &variant.ident,
                discriminant,
            });
        }

        Ok((default, fields))
    }
}
