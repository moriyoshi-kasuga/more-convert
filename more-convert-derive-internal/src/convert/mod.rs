use field::ConvertField;
use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Ident, ImplGenerics, TypeGenerics, WhereClause};
use target::ConvertTarget;

use crate::require_named_field_struct;

mod field;
mod field_arg;
mod from;
mod into;
mod target;

pub fn derive_convert(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let fields = require_named_field_struct(&input)?;

    let attr = 'bar: {
        for attr in &input.attrs {
            if attr.path().is_ident("convert") {
                break 'bar attr;
            }
        }
        return Err(syn::Error::new(
            input.span(),
            "expected `convert` attribute",
        ));
    };

    derive_convert_internal(&input, fields, attr)
}

fn derive_convert_internal(
    input: &syn::DeriveInput,
    fields: &syn::FieldsNamed,
    attr: &syn::Attribute,
) -> syn::Result<TokenStream> {
    let struct_targets = ConvertTarget::from_attr(attr)?;

    let fields = fields
        .named
        .iter()
        .map(ConvertField::from_field)
        .collect::<syn::Result<Vec<_>>>()?;

    for arg in &fields {
        for target in arg.target.keys() {
            if !struct_targets.iter().any(|v| v.check_inclusive(target)) {
                let (target, ident) = match target {
                    ConvertTarget::From(i) => ("from", i),
                    ConvertTarget::Into(i) => ("into", i),
                    ConvertTarget::FromInto(i) => ("from_into", i),
                };
                return Err(syn::Error::new(
                    ident.span(),
                    format!("`#[{}({})` is not in the target", target, ident),
                ));
            }
        }
    }

    let generics = input.generics.split_for_impl();

    struct_targets
        .into_iter()
        .map(|v| match v {
            ConvertTarget::From(ident) => Ok(gen_convert(
                &generics,
                &input.ident,
                &ident,
                &from::process_from(&ident, &fields)?,
            )),
            ConvertTarget::Into(ident) => Ok(gen_convert(
                &generics,
                &ident,
                &input.ident,
                &into::process_into(&ident, &fields)?,
            )),
            ConvertTarget::FromInto(ident) => {
                let mut into = gen_convert(
                    &generics,
                    &ident,
                    &input.ident,
                    &into::process_into(&ident, &fields)?,
                );
                let from = gen_convert(
                    &generics,
                    &input.ident,
                    &ident,
                    &from::process_from(&ident, &fields)?,
                );
                into.extend(from);

                Ok(into)
            }
        })
        .collect::<syn::Result<TokenStream>>()
}

pub(crate) enum GenType<'a> {
    Into(&'a Ident),
    From(&'a Ident),
}

pub(crate) fn gen_convert(
    (impl_generics, ty_generics, where_clause): &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    into_ident: &Ident,
    from_ident: &Ident,
    (idents, tokens): &(Vec<TokenStream>, Vec<TokenStream>),
) -> TokenStream {
    quote::quote! {
        impl #impl_generics std::convert::From<#from_ident> for #into_ident #ty_generics #where_clause {
            fn from(value: #from_ident) -> Self {
                Self {
                    #(
                        #idents: #tokens,
                    )*
                }
            }
        }
    }
}
