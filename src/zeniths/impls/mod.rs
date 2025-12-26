// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

#[cfg(feature = "c")]
pub mod c_zenith;
#[cfg(feature = "ini")]
pub mod ini_zenith;
#[cfg(feature = "java")]
pub mod java_zenith;
#[cfg(feature = "markdown")]
pub mod markdown_zenith;
#[cfg(feature = "prettier")]
pub mod prettier_zenith;
#[cfg(feature = "python")]
pub mod python_zenith;
#[cfg(feature = "rust")]
pub mod rust_zenith;
#[cfg(feature = "shell")]
pub mod shell_zenith;
#[cfg(feature = "toml")]
pub mod toml_zenith;
