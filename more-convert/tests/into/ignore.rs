use more_convert::Convert;

#[derive(Convert)]
#[convert(into(B))]
pub struct A {
    pub sample: u8,
    #[convert(ignore)]
    pub hey: u16,
}

pub struct B {
    sample: u8,
}

#[test]
pub fn main() {
    let a = A { sample: 1, hey: 2 };

    let b: B = a.into();

    assert_eq!(b.sample, 1);
}
