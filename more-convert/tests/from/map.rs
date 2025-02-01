use more_convert::Convert;

#[derive(Convert)]
#[convert(from(B))]
pub struct A {
    // value's type is `B`
    // The reason for the `value` is because of the From trait's args
    #[convert(map = value.map.to_string())]
    map: String,
    #[convert(map_field = map_field)]
    map_field: String,
    #[convert(map_struct = map_struct)]
    map_struct: String,
}

fn map_field(map_field: u8) -> String {
    map_field.to_string()
}

fn map_struct(b: &B) -> String {
    b.map_struct.to_string()
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
