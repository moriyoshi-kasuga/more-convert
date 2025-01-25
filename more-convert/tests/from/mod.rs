pub mod ignore;
pub mod map;
pub mod normal;
pub mod rename;

#[test]
pub fn normal() {
    normal::main();
}

#[test]
pub fn rename() {
    rename::main();
}

#[test]
pub fn ignore() {
    ignore::main();
}

#[test]
pub fn map() {
    map::main();
}
