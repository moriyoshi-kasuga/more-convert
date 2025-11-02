pub trait EnumRepr<T: Copy>: TryFrom<T> + Into<T> {}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
#[error("Failed to convert value {value} to enum {enum_name}")]
pub struct TryFromEnumReprError {
    pub enum_name: String,
    pub value: String,
}

impl TryFromEnumReprError {
    pub fn new(enum_name: String, value: String) -> Self {
        Self { enum_name, value }
    }
}
