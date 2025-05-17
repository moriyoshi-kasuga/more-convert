macro_rules! use_internal {
    ($internal:path, $input:ident) => {
        $internal(syn::parse_macro_input!($input as syn::DeriveInput))
            .unwrap_or_else(syn::Error::into_compile_error)
            .into()
    };
}

/// Automatically implements [`std::convert::From`] for repr from enum.
/// And implements [`std::convert::TryFrom`] for enum from repr.
/// or implements [`std::convert::From`] for enum from repr.
///
/// # Where to use:
///  - Managing Type
///  - Easy to understand constants
///
/// # Note:
///  - require `#[repr(u8)]` or `#[repr(u16)]` or ...
///  - default is require explicit
///  - #[enum_repr(default)] is special attribute on variant
///
/// # Enum Attribute:
///  - serde: automatically implements [`serde::Serialize`] and [`serde::Deserialize`]
///  - implicit: make it less explicit
///
/// # Variant Attribute:
///  - default: Sets the fallback value if none of the others apply.
///    this attribute is required to be used only once or not at all
///    if this attribute is not used, it will be impl [`std::convert::TryFrom`]
///    used, it will be impl [`std::convert::From`]
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
/// assert_eq!(TryInto::<Test>::try_into(1u8).unwrap_err(), more_convert::TryFromEnumReprError::new("Test".to_string(), 1.to_string()));
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
/// assert_eq!(serde_json::from_str::<Test>("1").unwrap_err().to_string(), String::from("Failed to convert value 1 to enum Test"));
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
/// assert_eq!(TryInto::<Test>::try_into(1u8).unwrap_err(), more_convert::TryFromEnumReprError::new("Test".to_string(), 1.to_string()));
/// ```
///
/// ## default
///
/// ```rust
/// use more_convert::EnumRepr;
/// #[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
/// #[repr(u8)]
/// pub enum Test {
///     Zero = 0,
///     One = 1,
///     #[enum_repr(default)]
///     Two = 2,
/// }
///
/// assert_eq!(0u8, Test::Zero.into());
/// assert_eq!(1u8, Test::One.into());
/// assert_eq!(2u8, Test::Two.into());
///
/// // Invalid values return the default variant
/// assert_eq!(Test::Two, 255u8.into());
///
#[proc_macro_derive(EnumRepr, attributes(enum_repr))]
pub fn derive_enum_repr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use_internal!(more_convert_derive_internal::derive_enum_repr, input)
}

/// Automatically implements [`std::convert::From`] on structs.
///
/// # Who uses it:
///   - When you are using the architectural
///   - Those who find the implementation of from and into cumbersome.
///
/// # Struct Attribute #[convert]:
///   - into: `impl From<#self> for #into_struct { /* auto gen */}`
///   - from: `impl From<#from_struct> for #self { /* auto gen */}`
///   - from_into: impl from and into
///
/// # Struct Attribute #[generate]:
///   - example `#[generate(B(is_negative = value.sample.is_negative()))]`
///     generate is used to generate the field value of the target struct
///
/// # Field Attribute:
///   - filter of target: (option, default apply all)
///     - example `#[convert(from(A,B),ignore,..)]`, this field ignored in A and B
///     - apply priority: from and into > from_into > all
///   - ignore: ignore this field
///   - rename: rename this field
///   - group of map: map this field (Choose one of these)
///     > default: `#field_name.into()`
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
/// ## from_into and filter
///
/// ```rust
/// use more_convert::Convert;
///
/// #[derive(Convert)]
/// #[convert(from_into(B))]
/// pub struct A {
///     pub sample: u8,
///
///     #[convert(from(B), map = Default::default())]
///     #[convert(into(B), ignore)]
///     pub hey: u16,
/// }
///
/// pub struct B {
///     sample: u8,
/// }
///
/// let b = B { sample: 1 };
/// let a: A = b.into();
/// assert_eq!(a.sample, 1u8);
/// assert_eq!(a.hey, 0u16);
/// ```
///
/// ## #[genearate]
///
/// ```rust
/// use more_convert::Convert;
///
/// #[derive(Convert)]
/// #[convert(into(B))]
/// #[generate(B(is_negative = value.sample.is_negative()))]
/// pub struct A {
///     pub sample: i8,
/// }
///
/// pub struct B {
///     pub sample: i8,
///     pub is_negative: bool,
/// }
///
/// let a = A { sample: -1 };
/// let b: B = a.into();
///
/// assert_eq!(b.sample, -1);
/// assert!(b.is_negative);
/// ```
///
#[proc_macro_derive(Convert, attributes(convert, generate))]
pub fn derive_convert(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use_internal!(more_convert_derive_internal::derive_convert, input)
}

/// Automatically implements [`more_convert::VariantName`] on enum
///
/// # Where to use:
///   - Only want the kind
///
/// # Note:
///   - prefix and suffix are applied after rename_all
///
/// # EnumAttribute:
///   - rename_all: apply rule to field name
///     - default is "none"
///     - The possible values are ("lowercase", "UPPERCASE", "PascalCase", "camelCase",
///       "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE")
///   - prefix: add prefix to field name
///   - suffix: add suffix to field name
///
/// # Variant Attribute:
///  - rename: rename field, (prefix, suffix and rename_all are not applied)
///  - nest: call VariantName on the first field of the variant
///
/// # Examples
///
/// ## Normal
///
/// ```rust
/// use more_convert::VariantName;
///
/// #[derive(VariantName)]
/// pub enum Error {
///   InvalidCode,
///   ServerError,
/// }
///
/// assert_eq!("InvalidCode", Error::InvalidCode.variant_name());
/// assert_eq!("ServerError", Error::ServerError.variant_name());
/// ```
///
/// ## rename and rename_all
///
/// ```rust
/// use more_convert::VariantName;
///
/// #[derive(VariantName)]
/// #[variant_name(rename_all = "snake_case")]
/// pub enum Error {
///   InvalidCode,
///   ServerError,
///   #[variant_name(rename = "NotFound")]
///   NotFoundError,
/// }
///
/// assert_eq!("invalid_code", Error::InvalidCode.variant_name());
/// assert_eq!("server_error", Error::ServerError.variant_name());
/// assert_eq!("NotFound", Error::NotFoundError.variant_name());
/// ```
///
/// ## prefix and suffix
/// ```rust
/// use more_convert::VariantName;
///
/// #[derive(VariantName)]
/// #[variant_name(prefix = "Error", suffix  = "What")]
/// pub enum Error {
///  InvalidCode,
///  ServerError,
/// }
///
/// assert_eq!("ErrorInvalidCodeWhat", Error::InvalidCode.variant_name());
/// assert_eq!("ErrorServerErrorWhat", Error::ServerError.variant_name());
/// ```
///
/// ## nest
/// ```rust
/// use more_convert::VariantName;
///
/// #[derive(VariantName)]
/// #[variant_name(prefix = "Inner")]
/// pub enum Inner {
///     A,
///     B,
/// }
///
/// #[derive(VariantName)]
/// pub enum TestVariantName {
///     InvalidCode,
///
///     #[variant_name(nest)]
///     Inner(Inner),
/// }
///
/// assert_eq!("InvalidCode", TestVariantName::InvalidCode.variant_name());
///
/// assert_eq!("InnerA", Inner::A.variant_name());
/// assert_eq!("InnerB", Inner::B.variant_name());
/// ```
///
/// ## without_trait
/// ```rust
/// #[derive(more_convert::VariantName)]
/// #[variant_name(without_trait)]
/// pub enum Error {
///  InvalidCode,
///  ServerError,
/// }
///
/// // call at const
/// const INVALID_CODE: &str = Error::InvalidCode.variant_name();
///
/// mod test {
///     fn something() {
///         // not depend on VariantName trait
///         crate::Error::InvalidCode.variant_name();
///     }
/// }
///
/// # fn main() {}
/// ```
#[proc_macro_derive(VariantName, attributes(variant_name))]
pub fn derive_variant_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use_internal!(more_convert_derive_internal::derive_variant_name, input)
}

/// Automatically generate a `COUNT` and `VARIANTS` constant for enum.
///
/// # Where to use:
/// - When you want to all variants of enum
/// - When you want to count of enum
///
/// # Note:
/// - `VARIANTS` order is same as enum definition order.
///   not calculate the discriminant value.
///
///
/// # EnumAttribute: none
/// # Variant Attribute: none
///
/// # Examples
///
/// ## Normal
///
/// ```rust
/// use more_convert::EnumArray;
/// #[derive(EnumArray, Clone, Copy, Debug, PartialEq)]
/// pub enum Test {
///     Zero = 1,
///     Three = 3,
///     Two = 2,
/// }
///
/// assert_eq!(Test::COUNT, 3usize);
/// // order is same as enum definition order.
/// // not calculate the discriminant value.
/// assert_eq!(Test::VARIANTS, &[Test::Zero, Test::Three, Test::Two]);
/// ```
#[proc_macro_derive(EnumArray, attributes(enum_array))]
pub fn derive_enum_array(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use_internal!(more_convert_derive_internal::derive_enum_array, input)
}
