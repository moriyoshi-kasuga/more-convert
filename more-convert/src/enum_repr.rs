#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
#[error("Failed to convert value {value} to enum {enum_name}")]
pub struct TryFromEnumReprError {
    pub enum_name: String,
    pub value: String,
}
