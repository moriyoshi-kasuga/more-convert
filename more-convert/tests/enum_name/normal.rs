use more_convert::EnumName;

#[derive(EnumName)]
pub enum TestEnumName {
    InvalidCode,
    B,
    HE { sample: u8 },
}

#[test]
pub fn main() {
    assert_eq!("InvalidCode", TestEnumName::InvalidCode.enum_name());
    assert_eq!("B", TestEnumName::B.enum_name());
    assert_eq!("HE", TestEnumName::HE { sample: 0 }.enum_name());
}
