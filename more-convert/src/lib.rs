#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]

pub use more_convert_derive::Convert;
pub use more_convert_derive::EnumArray;
pub use more_convert_derive::EnumRepr;
pub use more_convert_derive::VariantName;

mod enum_repr;
pub use enum_repr::*;

mod variant_name;
pub use variant_name::*;

#[cfg(feature = "utils")]
pub mod utils;
#[cfg(feature = "utils")]
pub use utils::*;
