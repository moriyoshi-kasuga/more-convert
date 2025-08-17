use proc_macro2::TokenStream;
use syn::Ident;

use crate::require_enum;

struct EnumArrayData<'a> {
    ident: &'a Ident,
    impl_generics: syn::ImplGenerics<'a>,
    ty_generics: syn::TypeGenerics<'a>,
    where_clause: Option<&'a syn::WhereClause>,
    variants: Vec<&'a Ident>,
}

impl<'a> EnumArrayData<'a> {
    fn from_input(input: &'a syn::DeriveInput) -> syn::Result<Self> {
        let variants = require_enum(input)?;
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        Ok(Self {
            ident: &input.ident,
            impl_generics,
            ty_generics,
            where_clause,
            variants: variants.iter().map(|f| &f.ident).collect(),
        })
    }
}

fn generate_code(data: EnumArrayData) -> TokenStream {
    let EnumArrayData {
        ident,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
    } = data;

    let count = variants.len();

    quote::quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub const COUNT: usize = #count;
            pub const VARIANTS: &[Self] = &[#(Self::#variants),*];
        }
    }
}

pub fn derive_enum_array(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let data = EnumArrayData::from_input(&input)?;
    Ok(generate_code(data))
}
