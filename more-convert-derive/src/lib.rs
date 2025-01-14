macro_rules! use_internal {
    ($internal:path, $input:ident) => {
        $internal(syn::parse_macro_input!($input as syn::DeriveInput))
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    };
}
