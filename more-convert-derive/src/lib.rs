macro_rules! use_internal {
    ($internal:path, $input:ident) => {
        $internal(syn::parse_macro_input!($input as syn::DeriveInput))
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    };
}

/// automatically implements [`std::convert::From`] and [`std::convert::Into`] for enums
///
/// # Where to use:
///  - Managing Type
///  - Easy to understand constants
///
/// # Note:
///  - require `#[repr(u8)]` or `#[repr(u16)]` or ...
///  - default is require explicit
///
/// # Enum Attribute:
///  - serde: automatically implements [`serde::Serialize`] and [`serde::Deserialize`]
///  - implicit: make it less explicit
///
/// # Variant Attribute:
///  ( Currency no. )
///
/// # Examples
///
/// ## Normal
///
/// ```rust
/// use more_convert::EnumRepr;
/// #[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
/// #[repr(u8)]
/// pub enum Test {
///     Zero = 0,
///     Three = 3,
///     Four = 4,
/// }
///
/// assert_eq!(0u8, Test::Zero.into());
/// assert_eq!(3u8, Test::Three.into());
/// assert_eq!(4u8, Test::Four.into());
///
/// assert_eq!(0u8.try_into(), Ok(Test::Zero));
/// assert_eq!(3u8.try_into(), Ok(Test::Three));
/// assert_eq!(4u8.try_into(), Ok(Test::Four));
///
/// assert_eq!(TryInto::<Test>::try_into(1u8).unwrap_err(), String::from("invalid Test: 1"));
/// ```
///
/// ## serde
///
/// ```rust
/// use more_convert::EnumRepr;
/// #[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
/// #[repr(u8)]
/// #[enum_repr(serde)]
/// pub enum Test {
///     Zero = 0,
///     Three = 3,
///     Four = 4,
/// }
///
///
/// assert_eq!(serde_json::to_string(&Test::Zero).unwrap(), "0");
/// assert_eq!(serde_json::to_string(&Test::Three).unwrap(), "3");
/// assert_eq!(serde_json::to_string(&Test::Four).unwrap(), "4");
///
/// assert_eq!(serde_json::from_str::<Test>("0").unwrap(), Test::Zero);
/// assert_eq!(serde_json::from_str::<Test>("3").unwrap(), Test::Three);
/// assert_eq!(serde_json::from_str::<Test>("4").unwrap(), Test::Four);
///
/// assert_eq!(serde_json::from_str::<Test>("1").unwrap_err().to_string(), String::from("invalid Test: 1"));
/// ```
/// ## implicit
///
/// ```rust
/// use more_convert::EnumRepr;
/// #[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
/// #[repr(u8)]
/// #[enum_repr(implicit)]
/// pub enum Test {
///     Zero,
///     Three = 3,
///     Four,
/// }
///
/// assert_eq!(0u8, Test::Zero.into());
/// assert_eq!(3u8, Test::Three.into());
/// assert_eq!(4u8, Test::Four.into());
///
/// assert_eq!(0u8.try_into(), Ok(Test::Zero));
/// assert_eq!(3u8.try_into(), Ok(Test::Three));
/// assert_eq!(4u8.try_into(), Ok(Test::Four));
///
/// assert_eq!(TryInto::<Test>::try_into(1u8).unwrap_err(), String::from("invalid Test: 1"));
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
