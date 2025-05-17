use more_convert::EnumArray;

#[derive(EnumArray, Clone, Copy, Debug, PartialEq)]
pub enum Test {
    Zero,
    Three = 3,
    Four,
}

#[test]
pub fn main() {
    assert_eq!(Test::COUNT, 3);
    assert_eq!(Test::VARIANTS, &[Test::Zero, Test::Three, Test::Four]);
}
