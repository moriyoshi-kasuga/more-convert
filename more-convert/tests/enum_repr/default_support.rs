use more_convert::EnumRepr;

#[derive(EnumRepr, Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
#[enum_repr(serde)]
pub enum Test {
    First = 1,
    Three = 3,
    #[enum_repr(default)]
    Four = 4,
}

#[cfg(test)]
fn test(origin: u16, v: Test) {
    let num: String = serde_json::to_string(&v).unwrap();
    assert_eq!(origin.to_string(), num);
    let test: Test = serde_json::from_str(&num).unwrap();
    assert_eq!(v, test);
}

#[test]
pub fn main() {
    test(1, Test::First);
    test(3, Test::Three);
    test(4, Test::Four);

    assert_eq!(
        serde_json::from_str::<Test>("0").unwrap_err().to_string(),
        String::from("invalid Test: 0")
    );
}
