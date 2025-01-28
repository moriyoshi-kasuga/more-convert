use more_convert::VecInto;

#[test]
fn test() {
    let v: Vec<u8> = vec![1u8, 2u8, 3u8];
    let v: Vec<u16> = v.vec_into();
    assert_eq!(v, vec![1u16, 2u16, 3u16]);
}

