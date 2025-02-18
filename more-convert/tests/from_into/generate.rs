use more_convert::Convert;

#[derive(Convert)]
#[convert(from_into(B))]
#[generate(B(sample = value.sample + 200))]
pub struct A {
    #[convert(into(B), ignore)]
    pub sample: u8,
}

#[derive(Debug, PartialEq)]
pub struct B {
    sample: u8,
}

#[test]
pub fn main() {
    let a = A { sample: 1 };

    let b: B = a.into();

    assert_eq!(b, B { sample: 201 });
}
