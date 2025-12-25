// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! Zenith 库的预导入 (prelude) 模块。
//! 该模块重新导出了一些频繁使用的类型和 Trait，以便于用户快速导入。

pub use crate::config::types::FormatResult;
pub use crate::config::types::ZenithConfig;
pub use crate::core::traits::Zenith;
pub use crate::error::{Result, ZenithError};
pub use crate::utils::path::{
    is_hidden, is_safe_path, is_safe_path_strict, sanitize_path_for_log, validate_path,
    validate_path_strict,
};
