macro_rules! use_internal {
    ($internal:path, $input:ident) => {
        $internal(syn::parse_macro_input!($input as syn::DeriveInput))
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    };
}

/// Automatically implements [`std::convert::From`] and [`std::convert::Into`] for repr on enums.
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

/// Automatically implements [`std::convert::From`] and [`std::convert::Into`] for repr on structs.
///
/// # Who uses it:
///   - When you are using the architectural
///   - Those who find the implementation of from and into cumbersome.
///
/// # Struct Attribute:
///   - group of convert: (Choose one of these)
///     - into: `impl From<#self> for #into_struct { /* auto gen */}`
///     - from: `impl From<#from_struct> for #self { /* auto gen */}`
///
/// # Field Attribute:
///   - ignore: ignore this field
///   - rename: rename this field
///   - group of map: map this field (Choose one of these)
///     - map: replace expr
///     - map_field: Process and pass field data
///     - map_struct: Create data from struct references
///
/// # Examples
///
/// ## Normal
///
/// ```rust
/// use more_convert::Convert;
/// #[derive(Convert)]
/// #[convert(into(B))]
/// pub struct A {
///     pub normal: u8,
///     // auto into of inner
///     pub opt: Option<u8>,
///     // auto into of inner
///     pub vec: Vec<u8>,
/// }
///
/// pub struct B {
///     normal: u16,
///     opt: Option<u16>,
///     vec: Vec<u16>,
/// }
///
/// let a = A {
///     normal: 0u8,
///     opt: Some(1u8),
///     vec: vec![2u8, 3u8],
/// };
///
/// let b: B = a.into();
///
/// assert_eq!(b.normal, 0u16);
/// assert_eq!(b.opt, Some(1u16));
/// assert_eq!(b.vec, vec![2u16, 3u16]);
/// ```
///
/// ## Reanem
///
/// ```rust
/// use more_convert::Convert;
/// #[derive(Convert)]
/// #[convert(into(B))]
/// pub struct A {
///     #[convert(rename = "sample")]
///     hey: String,
/// }
///
/// pub struct B {
///     sample: String,
/// }
///
/// let a = A {
///     hey: "hello".to_string(),
/// };
///
/// let b: B = a.into();
///
/// assert_eq!(b.sample, "hello");
/// ```
///
/// ## Map
///
/// ```rust
/// use more_convert::Convert;
/// #[derive(Convert)]
/// #[convert(into(B))]
/// pub struct A {
///     // value's type is `A`
///     // The reason for the `value` is because of the From trait's args
///     #[convert(map = value.map.to_string())]
///     map: u8,
///     #[convert(map_field = map_field)]
///     map_field: u8,
///     #[convert(map_struct = map_struct)]
///     map_struct: u8,
/// }
///
/// fn map_field(map_field: u8) -> String {
///     map_field.to_string()
/// }
///
/// fn map_struct(a: &A) -> String {
///     a.map_struct.to_string()
/// }
///
/// pub struct B {
///     map: String,
///     map_field: String,
///     map_struct: String,
/// }
///
/// let a = A {
///     map: 1,
///     map_field: 2,
///     map_struct: 3,
/// };
///
/// let b: B = a.into();
///
/// assert_eq!(b.map, "1");
/// assert_eq!(b.map_field, "2");
/// assert_eq!(b.map_struct, "3");
/// ```
///
#[proc_macro_derive(Convert, attributes(convert))]
pub fn derive_convert(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use_internal!(more_convert_derive_internal::derive_convert, input)
}
