use more_convert::EnumName;

#[derive(EnumName)]
#[enum_name(prefix = "Inner")]
pub enum Inner {
    A,
    B,
}

#[derive(EnumName)]
pub enum TestEnumName {
    InvalidCode,

    #[enum_name(nest)]
    Inner(Inner),
}

#[test]
pub fn main() {
    assert_eq!("InvalidCode", TestEnumName::InvalidCode.enum_name());

    assert_eq!("InnerA", Inner::A.enum_name());
    assert_eq!("InnerB", Inner::B.enum_name());
}
