use more_convert::Convert;

#[derive(Convert)]
#[convert(from_into(B))]
pub struct A {
    pub sample: u8,

    #[convert(from(B), map = Default::default())]
    #[convert(into(B), ignore)]
    pub hey: u16,
}

pub struct B {
    sample: u8,
}

#[test]
pub fn main() {
    let b = B { sample: 1 };
    let a: A = b.into();
    assert_eq!(a.sample, 1u8);
    assert_eq!(a.hey, 0u16);
}
