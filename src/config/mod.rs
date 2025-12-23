pub mod types;

use self::types::AppConfig;
use crate::error::{Result, ZenithError};
use config::{Config, Environment, File};
use std::path::PathBuf;

pub fn load_config(path: Option<PathBuf>) -> Result<AppConfig> {
    let mut builder = Config::builder();

    // 1. Load defaults (handled by struct defaults, but we can add a base layer here if needed)

    // 2. Load from file if provided, otherwise check default locations
    if let Some(p) = path {
        builder = builder.add_source(File::from(p).required(true));
    } else {
        // Try default locations
        let default_paths = vec!["zenith.toml", ".config/zenith/zenith.toml"];
        for p in default_paths {
            builder = builder.add_source(File::with_name(p).required(false));
        }
    }

    // 3. Load from Environment
    builder = builder.add_source(Environment::with_prefix("ZENITH").separator("_"));

    let config = builder
        .build()
        .map_err(|e| ZenithError::Config(e.to_string()))?;

    config
        .try_deserialize()
        .map_err(|e| ZenithError::Config(e.to_string()))
}
