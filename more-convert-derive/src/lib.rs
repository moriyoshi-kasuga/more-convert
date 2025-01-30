macro_rules! use_internal {
    ($internal:path, $input:ident) => {
        $internal(syn::parse_macro_input!($input as syn::DeriveInput))
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    };
}

/// automatically implements [`std::convert::From`] and [`std::convert::Into`] for enums
/// - Ideal for managing Type
/// - Easy to understand constants
///
/// # Example only derive
///
/// - require #[repr(u8)] or #[repr(u16)] or ...
/// - default is require explicit
///
/// ```rust
/// use more_convert::EnumRepr;
/// #[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
/// #[repr(u16)]
/// pub enum Test {
///     Zero = 0,
///     Three = 3,
///     Four = 4,
/// }
///
/// assert_eq!(0u16, Test::Zero.into());
/// assert_eq!(3u16, Test::Three.into());
/// assert_eq!(4u16, Test::Four.into());
///
/// assert_eq!(0u16.try_into(), Ok(Test::Zero));
/// assert_eq!(3u16.try_into(), Ok(Test::Three));
/// assert_eq!(4u16.try_into(), Ok(Test::Four));
/// ```
///
#[proc_macro_derive(EnumRepr, attributes(enum_repr))]
pub fn derive_enum_repr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use_internal!(more_convert_derive_internal::derive_enum_repr, input)
}

#[proc_macro_derive(Convert, attributes(convert))]
pub fn derive_convert(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use_internal!(more_convert_derive_internal::derive_convert, input)
}
