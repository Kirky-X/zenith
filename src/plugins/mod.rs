//! Plugin system for dynamic loading of formatting tools
//!
//! The plugin system allows users to extend Zenith's functionality with external formatting tools
//! through configuration files. Plugins can be loaded from JSON or TOML configuration files
//! that specify the command to run, arguments to pass, and file extensions to handle.
//!
//! # Example Plugin Configuration (JSON)
//!
//! ```json
//! {
//!   "name": "prettier-js",
//!   "command": "prettier",
//!   "args": ["--stdin-filepath", "{filepath}", "--parser", "babel"],
//!   "extensions": ["js", "jsx", "ts", "tsx"],
//!   "enabled": true
//! }
//! ```
//!
//! # Example Plugin Configuration (TOML)
//!
//! ```toml
//! name = "prettier-js"
//! command = "prettier"
//! args = ["--stdin-filepath", "{filepath}", "--parser", "babel"]
//! extensions = ["js", "jsx", "ts", "tsx"]
//! enabled = true
//! ```

pub mod loader;
pub mod types;

pub use loader::{PluginLoader, PluginSecurityConfig};
pub use types::PluginInfo;
