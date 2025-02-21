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

- **EnumName** provides a method to retrieve the name of an enum variant as a string.
  - This is particularly useful for error handling and logging, where understanding the
    specific variant can aid in debugging and reporting.
  - For more detailed information, please visit: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.EnumName.html)

## Examples

### EnumRepr

- enum_attributes
  - serde: automatically implements `serde::Serialize` and `serde::Deserialize`

more info: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.Convert.html)

```rust
use more_convert::EnumRepr;

#[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
#[enum_repr(implicit)]
pub enum Test {
    Zero,
    Three = 3,
    Four,
}

assert_eq!(0u8, Test::Zero.into());
assert_eq!(3u8, Test::Three.into());
assert_eq!(4u8, Test::Four.into());

assert_eq!(0u8.try_into(), Ok(Test::Zero));
assert_eq!(3u8.try_into(), Ok(Test::Three));
assert_eq!(4u8.try_into(), Ok(Test::Four));
```

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
    pub a: u8,
    pub b: u8,
}

pub struct B {
    pub a: u8,
    pub b: u8,
}

let b = B {
    a: 1u8,
    b: 2u8,
};

let a: A = b.into();

assert_eq!(a.a, 1u8);
assert_eq!(a.b, 2u8);
```

more `Into` examples are [here](./more-convert/tests/from/)  
more `From` examples are [here](./more-convert/tests/into/)

### EnumName

- enum_attributes

  - rename_all: apply rule to field name
    - Possible values: "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"
  - prefix: add prefix to field name
  - suffix: add suffix to field name

- variant_attributes
  - rename: rename field (prefix, suffix, and rename_all are not applied)

more info: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.EnumName.html)

```rust
use more_convert::EnumName;

#[derive(EnumName)]
pub enum Error {
    InvalidCode,
    ServerError,
}

assert_eq!("InvalidCode", Error::InvalidCode.enum_name());
assert_eq!("ServerError", Error::ServerError.enum_name());
```

## License

Licensed under

- [MIT license](https://github.com/moriyoshi-kasuga/more-convert/blob/main/LICENSE)
