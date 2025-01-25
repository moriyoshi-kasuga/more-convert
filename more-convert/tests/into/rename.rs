use more_convert::Convert;

#[derive(Convert)]
#[convert(into(B))]
pub struct A {
    #[convert(rename = "sample")]
    hey: String,
}

pub struct B {
    sample: String,
}

pub fn main() {
    let a = A {
        hey: "hello".to_string(),
    };

    let b: B = a.into();

    assert_eq!(b.sample, "hello");
}
