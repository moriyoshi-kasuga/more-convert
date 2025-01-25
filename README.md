# [more-convert][docsrs]: more convert utilities

[![more-convert on crates.io][cratesio-image]][cratesio]
[![more-convert on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/more-convert.svg
[cratesio]: https://crates.io/crates/more-convert
[docsrs-image]: https://docs.rs/more-convert/badge.svg
[docsrs]: https://docs.rs/more-convert

This crate provides utilities for convert

## Usage

`more-convert` provides a derive macro

- `EnumRepr` automatically implements `TryFrom` and `Into` for enums
  - Ideal for managing Type, etc.
  - Example: [test code](./more-convert/tests/enum_repr.rs)
- `Convert` automatically implements `From` or `Into` for named structs
  - Leave the very cumbersome From and Into implementations to us!
  - Example From: [from's test code](./more-convert/tests/from/normal.rs)
  - Example Into: [into's test code](./more-convert/tests/into/normal.rs)

## Example

### EnumRepr

```rust
use more_convert::EnumRepr;

#[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum Test {
    First,
    Three = 3,
    Four,
}

assert_eq!(0u16, Test::First.into());
assert_eq!(3u16, Test::Three.into());

assert_eq!(0u16.try_into(), Ok(Test::First));
assert_eq!(3u16.try_into(), Ok(Test::Three));
```

### Convert

```rust
use more_convert::Convert;

#[derive(Convert)]
#[convert(from(B))]
pub struct A {
    pub a: u16,
    pub b: u16,
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

assert_eq!(a.a, 1u16);
assert_eq!(a.b, 2u16);
```

more `Into` examples are [here](./more-convert/tests/from/)  
more `From` examples are [here](./more-convert/tests/into/)

## License

Licensed under

- [MIT license](https://github.com/moriyoshi-kasuga/more-convert/blob/main/LICENSE)
