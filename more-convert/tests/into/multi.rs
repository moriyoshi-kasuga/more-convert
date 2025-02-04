use more_convert::Convert;

#[derive(Convert)]
#[convert(from_into(B, C, D))]
pub struct A {
    #[convert(from_into(B, rename = "sample"))]
    #[convert(from_into(C, rename = "sample"))]
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

    let ab: A = b.into();
    let ac: A = c.into();

    assert_eq!(ab.hey, "B");
    assert_eq!(ac.hey, "C");

    let db: D = ab.into();
    let dc: D = ac.into();

    assert_eq!(db.hey, "B");
    assert_eq!(dc.hey, "C");
}
