use more_convert::Convert;

#[derive(Convert)]
#[convert(from(B))]
pub struct A {
    pub normal: u16,
    // auto into of inner
    pub opt: Option<u16>,
    // auto into of inner
    pub vec: Vec<u16>,
}

pub struct B {
    normal: u8,
    opt: Option<u8>,
    vec: Vec<u8>,
}

pub fn main() {
    let b = B {
        normal: 0,
        opt: Some(1),
        vec: vec![2, 3],
    };

    let a: A = b.into();

    assert_eq!(a.normal, 0u16);
    assert_eq!(a.opt, Some(1u16));
    assert_eq!(a.vec, vec![2u16, 3u16]);
}
