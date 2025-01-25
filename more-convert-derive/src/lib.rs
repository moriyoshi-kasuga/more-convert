macro_rules! use_internal {
    ($internal:path, $input:ident) => {
        $internal(syn::parse_macro_input!($input as syn::DeriveInput))
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    };
}

#[proc_macro_derive(EnumRepr)]
pub fn derive_enum_repr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use_internal!(more_convert_derive_internal::derive_enum_repr, input)
}

#[proc_macro_derive(Convert, attributes(convert))]
pub fn derive_convert(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use_internal!(more_convert_derive_internal::derive_convert, input)
}
