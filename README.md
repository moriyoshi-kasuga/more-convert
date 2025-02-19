# [more-convert][docsrs]: more convert utilities

[![more-convert on crates.io][cratesio-image]][cratesio]
[![more-convert on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/more-convert.svg
[cratesio]: https://crates.io/crates/more-convert
[docsrs-image]: https://docs.rs/more-convert/badge.svg
[docsrs]: https://docs.rs/more-convert

This crate provides utilities for convert

## Note

Don't worry, `into` attribute such as `Convert` implements `From` internally!

What I write below is what I picked up.
I try to keep it up to date in [doc.rs](https://docs.rs/more-convert/latest/more_convert), so please look there!

## Usage

`more-convert` provides a derive macro

- `Convert` automatically implements `From` for structs
  - Leave the very cumbersome From and Into implementations to us!
  - more info: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.Convert.html)
- `EnumRepr` Automatically implements [`std::convert::From`] for repr from enum.
  And implements [`std::convert::TryFrom`] for enum from repr.
  or implements [`std::convert::From`] for enum from repr.
  - Ideal for managing Type, etc.
  - more info: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.EnumRepr.html)
- `EnumName` provides a method to get the name of the enum variant
  - Ideal for error (kind) handling, etc.
  - more info: [doc.rs](https://docs.rs/more-convert/latest/more_convert/derive.EnumName.html)

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
