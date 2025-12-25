//! Common types and traits for the Zenith crate.
//! This prelude module exports frequently used items to make imports easier.

pub use crate::config::types::FormatResult;
pub use crate::config::types::ZenithConfig;
pub use crate::core::traits::Zenith;
pub use crate::error::{Result, ZenithError};
pub use crate::utils::path::{is_hidden, validate_path};
