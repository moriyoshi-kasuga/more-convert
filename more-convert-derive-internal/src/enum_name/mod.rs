use args::EnumNameFieldArg;
use proc_macro2::TokenStream;

use crate::require_enum;

mod args;

pub fn derive_enum_name(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let variants = require_enum(&input)?;
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let enum_arg = args::enum_attr(&input)?;

    let variant_name = variants.iter().map(|f| &f.ident).collect::<Vec<_>>();

    let variant_type = variants
        .iter()
        .map(|f| match f.fields {
            syn::Fields::Named(_) => quote::quote! { {..} },
            syn::Fields::Unnamed(_) => quote::quote! { {..} },
            syn::Fields::Unit => quote::quote! {},
        })
        .collect::<Vec<_>>();

    let body = variants
        .iter()
        .map(|v| args::variant_attr(v, &enum_arg))
        .collect::<syn::Result<Vec<_>>>()?
        .into_iter()
        .map(EnumNameFieldArg::into_token)
        .collect::<Vec<_>>();

    let token = quote::quote! {
        impl #impl_generics more_convert::EnumName for #ident #ty_generics #where_clause {
            fn enum_name(&self) -> &'static str {
                match self {
                    #(
                        #ident::#variant_name #variant_type => #body,
                    )*
                }
            }
        }
    };
    Ok(token)
}
