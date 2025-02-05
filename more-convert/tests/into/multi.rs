use more_convert::Convert;

#[derive(Convert)]
#[convert(into(B, C, D))]
pub struct A {
    #[convert(into(B, rename = "sample"))]
    #[convert(into(C, rename = "sample"))]
    hey: String,
}

pub struct B {
    sample: String,
}

pub struct C {
    sample: String,
}

pub struct D {
    hey: String,
}

#[test]
pub fn main() {
    let b = A {
        hey: "B".to_string(),
    };
    let c = A {
        hey: "C".to_string(),
    };
    let d = A {
        hey: "D".to_string(),
    };

    let ab: B = b.into();
    let ac: C = c.into();
    let ad: D = d.into();

    assert_eq!(ab.sample, "B");
    assert_eq!(ac.sample, "C");

    assert_eq!(ad.hey, "D");
}
