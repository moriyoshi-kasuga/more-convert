use convert_case::Case;
use syn::{meta::ParseNestedMeta, Type};

/// Validates that the input is a struct with named fields.
///
/// # Errors
///
/// Returns an error if the input is not a struct with named fields.
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
            "this macro only supports structs with named fields.\n\
            \n\
            Example: `struct Foo { field: Type }`",
        )),
    }
}

/// Validates that the input is an enum.
///
/// # Errors
///
/// Returns an error if the input is not an enum.
pub(crate) fn require_enum(
    input: &syn::DeriveInput,
) -> syn::Result<&syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>> {
    match &input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => Ok(variants),
        _ => Err(syn::Error::new_spanned(
            input,
            "this macro only supports enums",
        )),
    }
}

/// Checks if the given type is a `Vec<T>`.
///
/// # Examples
///
/// ```ignore
/// use syn::{parse_quote, Type};
/// let ty: Type = parse_quote!(Vec<u32>);
/// assert!(is_vec(&ty));
/// ```
pub(crate) fn is_vec(ty: &Type) -> bool {
    is_type_eq_ident(ty, "Vec")
}

/// Checks if the given type is an `Option<T>`.
///
/// # Examples
///
/// ```ignore
/// use syn::{parse_quote, Type};
/// let ty: Type = parse_quote!(Option<String>);
/// assert!(is_option(&ty));
/// ```
pub(crate) fn is_option(ty: &Type) -> bool {
    is_type_eq_ident(ty, "Option")
}

/// Checks if the last segment of a type path matches the given identifier.
///
/// # Examples
///
/// ```ignore
/// use syn::{parse_quote, Type};
/// let ty: Type = parse_quote!(std::vec::Vec<u32>);
/// assert!(is_type_eq_ident(&ty, "Vec"));
/// ```
pub(crate) fn is_type_eq_ident<S: AsRef<str>>(ty: &Type, s: S) -> bool {
    match get_last_path_segment(ty) {
        Some(seg) => seg.ident == s,
        _ => false,
    }
}

/// Extracts the last segment of a type path, if it exists.
///
/// # Examples
///
/// ```ignore
/// use syn::{parse_quote, Type};
/// let ty: Type = parse_quote!(std::vec::Vec<u32>);
/// let segment = get_last_path_segment(&ty);
/// assert_eq!(segment.unwrap().ident, "Vec");
/// ```
pub(crate) fn get_last_path_segment(ty: &Type) -> Option<&syn::PathSegment> {
    match ty {
        Type::Path(path) => path.path.segments.last(),
        _ => None,
    }
}

macro_rules! check_duplicate {
    ($span:expr, $variant:ident) => {
        $crate::check_duplicate!(@__message $span, $variant, $variant.is_some(),);
    };
    ($span:expr, $variant:ident, $additional:literal) => {
        $crate::check_duplicate!(@__message $span, $variant, $variant.is_some(), $additional);
    };
    ($span:expr, $variant:ident, $expr:expr) => {
        $crate::check_duplicate!(@__message $span, $variant, $expr,);
    };
    (@__message $span:expr, $variant:ident, $expr:expr, $($additional:expr)?) => {
        $crate::check_duplicate!(@__final $span, $expr, concat!("duplicate `", stringify!($variant), "` attribute.", $(" ", $additional)?));
    };
    (@__final $span:expr, $expr:expr, $message:expr) => {
        if $expr {
            return Err(syn::Error::new($span, $message));
        }
    };
}

pub(crate) use check_duplicate;

/// Removes the `r#` prefix from raw identifiers.
///
/// # Examples
///
/// ```ignore
/// use proc_macro2::Ident;
/// use quote::format_ident;
/// let ident = format_ident!("r#type");
/// assert_eq!(unraw(&ident), "type");
/// ```
pub(crate) fn unraw(ident: &proc_macro2::Ident) -> String {
    ident.to_string().trim_start_matches("r#").to_owned()
}

pub(crate) type AttrMetas = syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>;

pub(crate) fn parse_nested_attrs(
    name: &'static str,
    attrs: &[syn::Attribute],
    mut logic: impl FnMut(ParseNestedMeta) -> syn::Result<()>,
) -> syn::Result<bool> {
    let mut has_attr = false;
    for attr in attrs {
        if attr.path().is_ident(name) {
            has_attr = true;
            parse_nested_attr(attr, &mut logic)?;
        }
    }

    Ok(has_attr)
}

pub(crate) fn parse_nested_attr(
    attr: &syn::Attribute,
    logic: impl FnMut(ParseNestedMeta) -> syn::Result<()>,
) -> syn::Result<()> {
    attr.parse_nested_meta(logic)?;

    Ok(())
}

pub(crate) fn from_str_to_case(text: &str) -> Option<Case<'static>> {
    Some(match text {
        "lowercase" => Case::Lower,
        "UPPERCASE" => Case::Upper,
        "PascalCase" => Case::Pascal,
        "camelCase" => Case::Camel,
        "snake_case" => Case::Snake,
        "SCREAMING_SNAKE_CASE" => Case::UpperSnake,
        "kebab-case" => Case::Kebab,
        "SCREAMING-KEBAB-CASE" => Case::UpperKebab,
        _ => None?,
    })
}
