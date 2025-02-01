use more_convert::EnumRepr;

#[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
#[enum_repr(implicit)]
pub enum Test {
    Zero,
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
pub fn main() {
    test(0, Test::Zero);
    test(3, Test::Three);
    test(4, Test::Four);

    assert_eq!(
        TryInto::<Test>::try_into(1u16).unwrap_err(),
        String::from("invalid Test: 1")
    );
}
