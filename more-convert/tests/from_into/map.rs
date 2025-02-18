#![allow(clippy::unwrap_used)]

use more_convert::Convert;

#[derive(Convert)]
#[convert(from_into(B))]
pub struct A {
    #[convert(from(B), map_struct = map_struct_from_b)]
    #[convert(into(B), map_struct = map_struct_to_b)]
    map_struct: String,
    #[convert(from(B), map = value.map.to_string())]
    #[convert(into(B), map = value.map.parse().unwrap())]
    map: String,
    #[convert(from(B), map_field = map_field_from_b)]
    #[convert(into(B), map_field = map_field_to_b)]
    map_field: String,
}

fn map_field_from_b(map_field: u8) -> String {
    map_field.to_string()
}

fn map_field_to_b(map_field: String) -> u8 {
    map_field.parse().unwrap()
}

fn map_struct_from_b(b: &B) -> String {
    b.map_struct.to_string()
}

fn map_struct_to_b(a: &A) -> u8 {
    a.map_struct.parse().unwrap()
}

pub struct B {
    map: u8,
    map_field: u8,
    map_struct: u8,
}

#[test]
pub fn main() {
    let b = B {
        map: 1,
        map_field: 2,
        map_struct: 3,
    };

    let a: A = b.into();

    assert_eq!(a.map, "1");
    assert_eq!(a.map_field, "2");
    assert_eq!(a.map_struct, "3");
}
