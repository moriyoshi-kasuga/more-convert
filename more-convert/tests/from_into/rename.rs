use more_convert::Convert;

#[derive(Convert)]
#[convert(from_into(B))]
pub struct A {
    #[convert(from_into(B), rename = "sample")]
    hey: String,
}

pub struct B {
    sample: String,
}

#[test]
pub fn main() {
    let b = B {
        sample: "hello".to_string(),
    };

    let a: A = b.into();

    assert_eq!(a.hey, "hello");
}
