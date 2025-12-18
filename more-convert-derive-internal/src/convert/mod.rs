use field::ConvertField;
use generate::GenerateArg;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Ident, ImplGenerics, TypeGenerics, WhereClause};
use target::{Conversion, ConvertArgs};

use crate::require_named_field_struct;

mod field;
mod field_arg;
mod generate;
mod target;

pub fn derive_convert(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let self_ident = &input.ident;
    let fields = require_named_field_struct(&input)?;

    // 1. Parse attributes to get a list of conversions and generations
    let mut conversions = Vec::new();
    let mut generates: Vec<GenerateArg> = vec![];
    for attr in &input.attrs {
        if attr.path().is_ident("convert") {
            let args: ConvertArgs = attr.parse_args()?;
            conversions.extend(args.into_conversions(self_ident)?);
        }
        if attr.path().is_ident("generate") {
            generates.push(attr.parse_args()?);
        }
    }

    // 2. Parse field information
    let fields = fields
        .named
        .iter()
        .map(|f| ConvertField::from_field(f, self_ident))
        .collect::<syn::Result<Vec<_>>>()?;

    // 3. Validate field-level attributes
    validate_field_attributes(&fields, &conversions, &generates)?;

    // 4. Generate `impl From` for each conversion
    let generics = input.generics.split_for_impl();
    conversions
        .into_iter()
        .map(|conversion| gen_from_impl(&generics, &conversion, &fields, &generates, self_ident))
        .collect::<syn::Result<TokenStream>>()
}

fn validate_field_attributes(
    fields: &[ConvertField],
    conversions: &[Conversion],
    generates: &[GenerateArg],
) -> syn::Result<()> {
    for field in fields {
        for conv in field.target.keys() {
            if !conversions.contains(conv) {
                return Err(syn::Error::new(
                    field.ident.span(),
                    format!(
                        "field '{}' specifies `convert` for `{} -> {}`, but the struct-level attribute is missing.\n\
                        \n\
                        Help: Add `#[convert(from({}))]` or `#[convert(into({}))]` to the struct definition.",
                        field.ident, conv.from, conv.to, conv.from, conv.to
                    ),
                ));
            }
        }
    }

    for generate in generates {
        if !conversions.iter().any(|c| c.to == generate.into_ident) {
            return Err(syn::Error::new(
                generate.into_ident.span(),
                format!(
                    "`generate` for `{}` is specified, but there is no `into({})` or `from_into({})` attribute",
                    generate.into_ident, generate.into_ident, generate.into_ident
                ),
            ));
        }
    }

    Ok(())
}

fn gen_from_impl(
    generics: &(ImplGenerics, TypeGenerics, Option<&WhereClause>),
    conversion: &Conversion,
    fields: &[ConvertField],
    generates: &[GenerateArg],
    self_ident: &Ident,
) -> syn::Result<TokenStream> {
    let from_ident = &conversion.from;
    let into_ident = &conversion.to;

    let mut field_idents = Vec::new();
    let mut field_tokens = Vec::new();

    // Handle `generate` attributes for `Into` conversions
    if from_ident == self_ident {
        for g in generates {
            if g.into_ident == *into_ident {
                field_idents.push(g.field_ident.to_token_stream());
                field_tokens.push(g.expr.to_token_stream());
            }
        }
    }

    for field in fields {
        let arg = field.get_arg_for_conversion(conversion);

        if arg.ignore {
            continue;
        }

        let (target_field_ident, source_field_ident) = if *into_ident == *self_ident {
            // impl From<T> for Self
            let source_name = arg
                .rename
                .as_ref()
                .map(|l| l.value())
                .unwrap_or_else(|| field.ident.to_string());
            (
                field.ident.to_token_stream(),
                Ident::new(&source_name, field.ident.span()).to_token_stream(),
            )
        } else {
            // impl From<Self> for T
            let target_name = arg
                .rename
                .as_ref()
                .map(|l| l.value())
                .unwrap_or_else(|| field.ident.to_string());
            (
                Ident::new(&target_name, field.ident.span()).to_token_stream(),
                field.ident.to_token_stream(),
            )
        };

        let token = arg.map.to_token(&source_field_ident);

        field_idents.push(target_field_ident);
        field_tokens.push(token);
    }

    let (impl_generics, ty_generics, where_clause) = generics;
    Ok(quote::quote! {
        impl #impl_generics std::convert::From<#from_ident> for #into_ident #ty_generics #where_clause {
            fn from(value: #from_ident) -> Self {
                Self {
                    #( #field_idents: #field_tokens, )*
                }
            }
        }
    })
}
