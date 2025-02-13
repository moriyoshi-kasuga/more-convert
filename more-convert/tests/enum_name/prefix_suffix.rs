use more_convert::EnumName;

#[derive(EnumName)]
#[enum_name(prefix = "Prefix_", suffix = "_Suffix")]
pub enum TestEnumName {
    InvalidCode,
    B,
    He { sample: u8 },
}

#[test]
pub fn main() {
    assert_eq!(
        "Prefix_InvalidCode_Suffix",
        TestEnumName::InvalidCode.enum_name()
    );
    assert_eq!("Prefix_B_Suffix", TestEnumName::B.enum_name());
    assert_eq!(
        "Prefix_He_Suffix",
        TestEnumName::He { sample: 0 }.enum_name()
    );
}
