use more_convert::EnumName;

#[derive(EnumName)]
#[enum_name(rename_all = "snake_case", without_trait)]
pub enum TestEnumName {
    InvalidCode,
    B,
    He { sample: u8 },
}

#[test]
pub fn main() {
    assert_eq!("invalid_code", TestEnumName::InvalidCode.enum_name());
    assert_eq!("b", TestEnumName::B.enum_name());
    assert_eq!("he", TestEnumName::He { sample: 0 }.enum_name());
}
