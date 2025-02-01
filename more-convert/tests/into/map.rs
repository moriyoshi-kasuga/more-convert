use more_convert::Convert;

#[derive(Convert)]
#[convert(into(B))]
pub struct A {
    // value's type is `A`
    // The reason for the `value` is because of the From trait's args
    #[convert(map = value.map.to_string())]
    map: u8,
    #[convert(map_field = map_field)]
    map_field: u8,
    #[convert(map_struct = map_struct)]
    map_struct: u8,
}

fn map_field(map_field: u8) -> String {
    map_field.to_string()
}

fn map_struct(a: &A) -> String {
    a.map_struct.to_string()
}

pub struct B {
    map: String,
    map_field: String,
    map_struct: String,
}

#[test]
pub fn main() {
    let a = A {
        map: 1,
        map_field: 2,
        map_struct: 3,
    };

    let b: B = a.into();

    assert_eq!(b.map, "1");
    assert_eq!(b.map_field, "2");
    assert_eq!(b.map_struct, "3");
}
