#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]

pub use more_convert_derive::Convert;
pub use more_convert_derive::EnumRepr;
pub use more_convert_derive::EnumName;

pub mod enum_name;
pub use enum_name::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "utils")] {
        pub mod utils;
        pub use utils::*;
    }
}
