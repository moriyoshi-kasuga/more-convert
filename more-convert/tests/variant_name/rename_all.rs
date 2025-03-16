use more_convert::VariantName;

#[derive(VariantName)]
#[variant_name(rename_all = "snake_case")]
pub enum TestVariantName {
    InvalidCode,
    B,
    He { sample: u8 },
}

#[test]
pub fn main() {
    assert_eq!("invalid_code", TestVariantName::InvalidCode.variant_name());
    assert_eq!("b", TestVariantName::B.variant_name());
    assert_eq!("he", TestVariantName::He { sample: 0 }.variant_name());
}
