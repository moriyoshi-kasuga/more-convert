use more_convert::Convert;

#[derive(Convert)]
#[convert(into(B))]
pub struct A {
    pub normal: u8,
    // auto into of inner
    pub opt: Option<u8>,
    // auto into of inner
    pub vec: Vec<u8>,
}

pub struct B {
    normal: u16,
    opt: Option<u16>,
    vec: Vec<u16>,
}

#[test]
pub fn main() {
    let a = A {
        normal: 0,
        opt: Some(1),
        vec: vec![2, 3],
    };

    let b: B = a.into();

    assert_eq!(b.normal, 0u16);
    assert_eq!(b.opt, Some(1u16));
    assert_eq!(b.vec, vec![2u16, 3u16]);
}
