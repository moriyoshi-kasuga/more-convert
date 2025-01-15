use more_convert::EnumRepr;

fn main() {}

#[derive(EnumRepr)]
#[repr(u16)]
pub enum Test {
    First,
    Three = 3,
    Four,
}
