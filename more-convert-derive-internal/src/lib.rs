mod enum_repr;
pub use enum_repr::*;

mod convert;
pub use convert::*;

use syn::Type;

pub(crate) fn require_named_field_struct(
    input: &syn::DeriveInput,
) -> syn::Result<&syn::FieldsNamed> {
    match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => Ok(fields),
        _ => Err(syn::Error::new_spanned(
            input,
            "currently only structs with named fields are supported",
        )),
    }
}

pub(crate) fn require_enum(
    input: &syn::DeriveInput,
) -> syn::Result<&syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>> {
    match &input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => Ok(variants),
        _ => Err(syn::Error::new_spanned(
            input,
            "currently only enums are supported",
        )),
    }
}

pub(crate) fn is_vec(ty: &Type) -> bool {
    is_type_eq_ident(ty, "Vec")
}

pub(crate) fn is_option(ty: &Type) -> bool {
    is_type_eq_ident(ty, "Option")
}

pub(crate) fn is_type_eq_ident<S: AsRef<str>>(ty: &Type, s: S) -> bool {
    match get_last_path_segment(ty) {
        Some(seg) => seg.ident == s,
        _ => false,
    }
}

pub(crate) fn get_last_path_segment(ty: &Type) -> Option<&syn::PathSegment> {
    match ty {
        Type::Path(path) => path.path.segments.last(),
        _ => None,
    }
}
