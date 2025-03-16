use more_convert::VariantName;

#[derive(VariantName)]
#[variant_name(prefix = "Prefix_", suffix = "_Suffix")]
pub enum TestVariantName {
    InvalidCode,
    B,
    He { sample: u8 },
}

#[test]
pub fn main() {
    assert_eq!(
        "Prefix_InvalidCode_Suffix",
        TestVariantName::InvalidCode.variant_name()
    );
    assert_eq!("Prefix_B_Suffix", TestVariantName::B.variant_name());
    assert_eq!(
        "Prefix_He_Suffix",
        TestVariantName::He { sample: 0 }.variant_name()
    );
}
