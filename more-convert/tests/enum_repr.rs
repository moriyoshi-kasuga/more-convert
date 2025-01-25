use more_convert::EnumRepr;

#[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum Test {
    First,
    Three = 3,
    Four,
}

#[cfg(test)]
fn test(origin: u16, v: Test) {
    let num: u16 = v.into();
    assert_eq!(origin, num);
    let test: Test = num.try_into().unwrap();
    assert_eq!(v, test);
}

#[test]
fn first() {
    test(0, Test::First);
}

#[test]
fn three() {
    test(3, Test::Three);
}

#[test]
fn four() {
    test(4, Test::Four);
}
