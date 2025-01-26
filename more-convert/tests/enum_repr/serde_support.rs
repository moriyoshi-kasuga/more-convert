use more_convert::EnumRepr;

#[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
#[enum_repr(serde)]
pub enum Test {
    First,
    Three = 3,
    Four,
}

#[cfg(test)]
fn test(origin: u16, v: Test) {
    let num: String = serde_json::to_string(&v).unwrap();
    assert_eq!(origin.to_string(), num);
    let test: Test = serde_json::from_str(&num).unwrap();
    assert_eq!(v, test);
}

pub fn main() {
    test(0, Test::First);
    test(3, Test::Three);
    test(4, Test::Four);
}
