use crate::error::Result;
use crate::zeniths::registry::ZenithRegistry;
use std::process::Command;
use std::sync::Arc;

pub struct EnvironmentChecker;

pub struct ToolStatus {
    pub name: String,
    pub available: bool,
    pub version: Option<String>,
}

impl EnvironmentChecker {
    pub fn check_tool(tool: &str) -> ToolStatus {
        match Command::new(tool).arg("--version").output() {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .next()
                    .map(|s| s.trim().to_string());
                ToolStatus {
                    name: tool.to_string(),
                    available: true,
                    version,
                }
            }
            _ => ToolStatus {
                name: tool.to_string(),
                available: false,
                version: None,
            },
        }
    }

    pub fn check_all(registry: Arc<ZenithRegistry>) -> Vec<ToolStatus> {
        let mut tools = std::collections::HashSet::new();
        for zenith in registry.list_all() {
            tools.insert(zenith.name().to_string());
        }

        let mut results = Vec::new();
        for tool in tools {
            results.push(Self::check_tool(&tool));
        }
        results.sort_by(|a, b| a.name.cmp(&b.name));
        results
    }
}
