use more_convert::EnumName;

#[derive(EnumName)]
#[enum_name(rename_all = "snake_case", prefix = "Prefix_", suffix = "_Suffix")]
pub enum TestEnumName {
    InvalidCode,
    B,
    He { sample: u8 },
}

#[test]
pub fn main() {
    assert_eq!(
        "Prefix_invalid_code_Suffix",
        TestEnumName::InvalidCode.enum_name()
    );
    assert_eq!("Prefix_b_Suffix", TestEnumName::B.enum_name());
    assert_eq!(
        "Prefix_he_Suffix",
        TestEnumName::He { sample: 0 }.enum_name()
    );
}
