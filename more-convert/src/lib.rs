#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]

pub use more_convert_derive::Convert;
pub use more_convert_derive::EnumName;
pub use more_convert_derive::EnumRepr;

pub mod enum_name;
pub use enum_name::EnumName;

#[cfg(feature = "utils")]
pub mod utils;
#[cfg(feature = "utils")]
pub use utils::*;
