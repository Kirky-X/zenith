//! Plugin-related types and definitions
//!
//! This module contains type definitions used throughout the plugin system,
//! including information about loaded plugins and configuration structures.

/// Information about a loaded plugin
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub extensions: Vec<String>,
}

/// Configuration for plugin loading
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub enabled: bool,
    pub path: String,
    pub auto_update: bool,
}
