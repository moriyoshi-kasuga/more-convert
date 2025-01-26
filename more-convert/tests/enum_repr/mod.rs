pub mod normal;
pub mod serde_support;

#[test]
fn normal() {
    normal::main();
}

#[test]
fn serde_support() {
    serde_support::main();
}
