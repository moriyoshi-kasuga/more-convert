# [more-convert][docsrs]: more convert utilities

[![more-convert on crates.io][cratesio-image]][cratesio]
[![more-convert on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/more-convert.svg
[cratesio]: https://crates.io/crates/more-convert
[docsrs-image]: https://docs.rs/more-convert/badge.svg
[docsrs]: https://docs.rs/more-convert

This crate provides utilities for convert

## Note

The `Convert` attribute guarantees that the `into` method automatically implements the `From` trait internally, ensuring seamless conversions!

The information provided below is a summary of key points.
For the most current and detailed documentation, please refer to [doc.rs](https://docs.rs/more-convert/latest/more_convert).

## Usage

`more-convert` provides a derive macro

- **Convert**:
  - This macro is designed to handle simple conversions by automatically implementing
    the `From` trait for structs.
  - It aims to eliminate boilerplate code within your architecture, focusing on
    straightforward use cases.
  - For more detailed information, please visit: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.Convert.html)

- **EnumRepr**:
  - This macro primarily implements the `TryFrom` trait for safe conversions from the
    representation type back to the enum. This ensures that conversions are explicitly
    handled and potential errors are managed.
  - Optionally, the `From` trait can be implemented for converting an enum to its
    representation type when a default value is specified using the `#[enum_repr(default)]`
    attribute. This provides a fallback mechanism for conversions.
  - By using enums instead of primitive types like `u8`, it enhances code readability and
    maintainability, making it easier to manage types and ensure type safety in conversions.
  - For more detailed information, please visit: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.EnumRepr.html)

- **VariantName** provides a method to retrieve the name of an enum variant as a string.
  - This is particularly useful for error handling and logging, where understanding the
    specific variant can aid in debugging and reporting.
  - For more detailed information, please visit: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.VariantName.html)

- **EnumArray**
  - It automatically implements the `COUNT` and `VARIANTS` const, which return the number of
    variants and an array of all variants, respectively.
  - This is useful when you need to iterate over every possible value of
    an enum or perform operations on each variant.

## Examples

### Convert

- field_attributes
  - ignore: skip the field
  - rename: rename the field
  - map
    - map: map of expr
    - map_field: map of field
    - map_struct: map of struct

more info: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.EnumRepr.html)

```rust
use more_convert::Convert;

#[derive(Convert)]
#[convert(from(B))]
pub struct A {
    #[convert(map = value.sample.to_string())]
    pub sample: String,
    // auto into of inner
    pub opt: Option<u16>,
    // auto into of inner
    pub vec: Vec<u16>,
}

pub struct B {
    pub sample: u8,
    pub opt: Option<u8>,
    pub vec: Vec<u8>,
}

let b = B {
    sample: 1u8,
    opt: Some(0u8),
    vec: vec![1u8, 2u8],
};

let a: A = b.into();

assert_eq!(a.sample, String::from("1"));
assert_eq!(a.opt, Some(0u16));
assert_eq!(a.vec, vec![1u16, 2u16]);
```

### EnumRepr

- enum_attributes
  - serde: automatically implements `serde::Serialize` and `serde::Deserialize`
  - implicit: not required to specify the discriminant (not recommended)

- variant_attributres
  - default: set the fallback value for the `From` trait
  
more info: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.Convert.html)

#### impled TryFrom (not use default attribute)

```rust
use more_convert::EnumRepr;

#[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
#[enum_repr(implicit, serde)]
#[repr(u8)]
pub enum Test {
    Zero,
    Three = 3,
    Four,
}

assert_eq!(u8::from(Test::Zero), 0u8);
assert_eq!(serde_json::to_string(&Test::Zero).unwrap(), "0");

assert_eq!(serde_json::from_str::<Test>("0").unwrap(), Test::Zero);

// return error with unknown value 
assert_eq!(Test::try_from(1).unwrap_err().to_string(), String::from("Failed to convert value 1 to enum Test"));
assert_eq!(serde_json::from_str::<Test>("1").unwrap_err().to_string(), String::from("Failed to convert value 1 to enum Test"));
```

#### impled From (use default attribute)

```rust
use more_convert::EnumRepr;

#[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
#[enum_repr(implicit, serde)]
#[repr(u8)]
pub enum Test {
  #[enum_repr(default)]
  Zero,
  Three = 3,
  Four,
}

// return fallback with unknown value

// impled From
assert_eq!(Test::Zero, 1u8.into());
assert_eq!(serde_json::from_str::<Test>("1").unwrap(), Test::Zero);

```

### VariantName

- enum_attributes

  - without_tarit: without `more_convert::VariantName` trait and definition fn as `const` (and the caller does not need to depend on this crate)
  - rename_all: apply rule to field name
    - Possible values: "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"
  - prefix: add prefix to field name
  - suffix: add suffix to field name

- variant_attributes
  - rename: rename field (prefix, suffix, and rename_all are not applied)

more info: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.VariantName.html)

```rust
use more_convert::VariantName;

// not apply rename_all to prefix, Don't forget the underscore.
#[derive(VariantName)]
#[variant_name(rename_all = "snake_case", prefix = "error_")]
pub enum Error {
    InvalidCode,
    ServerError,
}

assert_eq!("error_invalid_code", Error::InvalidCode.variant_name());
assert_eq!("error_server_error", Error::ServerError.variant_name());
```

## EnumArray

- enum_attributes: none
- variant_attributes: none

```rust
use more_convert::EnumArray;

#[derive(Debug, PartialEq, EnumArray)]
pub enum Test {
    Zero,
    Two,
    Three,
}

assert_eq!(Test::COUNT, 3usize);
assert_eq!(Test::VARIANTS, &[Test::Zero, Test::Two, Test::Three]);
```

## License

Licensed under

- [MIT license](https://github.com/moriyoshi-kasuga/more-convert/blob/main/LICENSE)
