use more_convert::Convert;

#[derive(Convert)]
#[convert(into(B))]
#[generate(B(hey = value.sample.to_string()))]
pub struct A {
    pub sample: u8,
}

#[derive(Debug, PartialEq)]
pub struct B {
    sample: u8,
    hey: String,
}

#[test]
pub fn main() {
    let a = A { sample: 1 };

    let b: B = a.into();

    assert_eq!(
        b,
        B {
            sample: 1,
            hey: String::from("1"),
        }
    );
}
