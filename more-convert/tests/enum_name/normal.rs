use more_convert::EnumName;

#[derive(EnumName)]
#[enum_name(rename_all = "snake_case", prefix = "Test")]
pub enum TestEnumName {
    InvalidCode,
    B,
    HE { sample: u8 },
}

#[test]
pub fn main() {
    let name: &'static str = TestEnumName::InvalidCode.enum_name();

    assert_eq!("test_invalid_code", name);
}
