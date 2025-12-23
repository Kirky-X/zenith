use crate::error::Result;
use std::process::Command;

pub struct EnvironmentChecker;

impl EnvironmentChecker {
    pub fn check_tool(tool: &str) -> Result<bool> {
        match Command::new(tool).arg("--version").output() {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        }
    }
}
