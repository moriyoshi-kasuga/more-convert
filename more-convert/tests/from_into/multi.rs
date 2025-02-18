use more_convert::Convert;

#[derive(Convert)]
#[convert(from_into(B, C, D))]
pub struct A {
    #[convert(from_into(B, C), rename = "sample")]
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
    let b = B {
        sample: "B".to_string(),
    };
    let c = C {
        sample: "C".to_string(),
    };
    let d = D {
        hey: "D".to_string(),
    };

    let ab: A = b.into();
    let ac: A = c.into();
    let ad: A = d.into();

    assert_eq!(ab.hey, "B");
    assert_eq!(ac.hey, "C");
    assert_eq!(ad.hey, "D");
}
