use more_convert::VariantName;

#[derive(VariantName)]
pub enum TestVariantName {
    InvalidCode,
    B,
    HE { sample: u8 },
}

#[test]
pub fn main() {
    assert_eq!("InvalidCode", TestVariantName::InvalidCode.variant_name());
    assert_eq!("B", TestVariantName::B.variant_name());
    assert_eq!("HE", TestVariantName::HE { sample: 0 }.variant_name());
}
