use syn::{meta::ParseNestedMeta, Meta, Type};

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

pub(crate) fn require_lit_str<S: syn::spanned::Spanned>(
    span: &S,
    expr: &syn::Expr,
) -> syn::Result<String> {
    if let syn::Expr::Lit(expr_lit) = &expr {
        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
            return Ok(lit_str.value());
        }
    }

    Err(syn::Error::new(span.span(), "expected string literal"))
}

pub(crate) fn unraw(ident: &proc_macro2::Ident) -> String {
    ident.to_string().trim_start_matches("r#").to_owned()
}

pub(crate) type AttrMetas = syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>;

pub(crate) fn parse_meta_attrs<F>(
    name: &'static str,
    attrs: &[syn::Attribute],
    mut func: F,
) -> syn::Result<bool>
where
    F: FnMut(Meta) -> syn::Result<()>,
{
    let mut has_attr = false;
    for attr in attrs {
        if attr.path().is_ident(name) {
            has_attr = true;
            parse_meta_attr(attr, &mut func)?;
        }
    }

    Ok(has_attr)
}

pub(crate) fn parse_meta_attr<F>(attr: &syn::Attribute, mut func: F) -> syn::Result<()>
where
    F: FnMut(Meta) -> syn::Result<()>,
{
    let nested = attr.parse_args_with(AttrMetas::parse_terminated)?;

    for meta in nested {
        func(meta)?;
    }

    Ok(())
}

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
