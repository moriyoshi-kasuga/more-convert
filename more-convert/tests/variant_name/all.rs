use more_convert::VariantName;

#[derive(VariantName)]
#[variant_name(rename_all = "snake_case", prefix = "Prefix_", suffix = "_Suffix")]
pub enum TestVariantName {
    InvalidCode,
    B,
    He { sample: u8 },
}

#[test]
pub fn main() {
    assert_eq!(
        "Prefix_invalid_code_Suffix",
        TestVariantName::InvalidCode.variant_name()
    );
    assert_eq!("Prefix_b_Suffix", TestVariantName::B.variant_name());
    assert_eq!(
        "Prefix_he_Suffix",
        TestVariantName::He { sample: 0 }.variant_name()
    );
}
