use more_convert::VariantName;

#[derive(VariantName)]
#[variant_name(prefix = "Inner")]
pub enum Inner {
    A,
    B,
}

#[derive(VariantName)]
pub enum TestVariantName {
    InvalidCode,

    #[variant_name(nest)]
    Inner(Inner),
}

#[test]
pub fn main() {
    assert_eq!("InvalidCode", TestVariantName::InvalidCode.variant_name());

    assert_eq!("InnerA", Inner::A.variant_name());
    assert_eq!("InnerB", Inner::B.variant_name());
}
