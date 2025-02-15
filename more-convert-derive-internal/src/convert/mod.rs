use args::{ConvertArgs, ConvertFieldArgKind, ConvertFieldArgs};
use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Ident, ImplGenerics, TypeGenerics, WhereClause};

use crate::require_named_field_struct;

mod args;
mod from;
mod into;

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

    let struct_args = ConvertArgs::from_attr(attr)?;

    let fields = fields
        .named
        .iter()
        .map(ConvertFieldArgs::from_field)
        .collect::<syn::Result<Vec<_>>>()?;

    for field in &fields {
        for arg in &field.arg {
            match &arg.kind {
                ConvertFieldArgKind::From(kind_ident) => {
                    let has = struct_args
                        .iter()
                        .find(|v| matches!(v, ConvertArgs::From(ident) if ident == kind_ident));
                    if has.is_none() {
                        return Err(syn::Error::new(
                            kind_ident.span(),
                            format!("`{}` is not in the `from` target", kind_ident),
                        ));
                    }
                }
                ConvertFieldArgKind::Into(kind_ident) => {
                    let has = struct_args
                        .iter()
                        .find(|v| matches!(v, ConvertArgs::Into(ident) if ident == kind_ident));
                    if has.is_none() {
                        return Err(syn::Error::new(
                            kind_ident.span(),
                            format!("`{}` is not in the `into` target", kind_ident),
                        ));
                    }
                }
                ConvertFieldArgKind::All => continue,
            }
        }
    }

    let generics = input.generics.split_for_impl();

    struct_args
        .into_iter()
        .map(|v| match v {
            ConvertArgs::From(ident) => Ok(gen_convert(
                &generics,
                &input.ident,
                &ident,
                &from::process_from(&ident, &fields)?,
            )),
            ConvertArgs::Into(ident) => Ok(gen_convert(
                &generics,
                &ident,
                &input.ident,
                &into::process_into(&ident, &fields)?,
            )),
        })
        .collect::<syn::Result<TokenStream>>()
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
