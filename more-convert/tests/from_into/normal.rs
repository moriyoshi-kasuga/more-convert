use more_convert::Convert;

#[derive(Convert)]
#[convert(from_into(SampleB))]
pub struct SampleA {
    pub test: String,
}

pub struct SampleB {
    pub test: String,
}

#[test]
pub fn main() {
    let b = SampleB {
        test: "hello".to_string(),
    };

    let a: SampleA = b.into();

    assert_eq!(a.test, "hello");

    let b: SampleB = a.into();

    assert_eq!(b.test, "hello");
}
