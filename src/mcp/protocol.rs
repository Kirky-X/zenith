// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct FormatParams {
    pub paths: Vec<PathBuf>,
    #[serde(default)]
    pub recursive: bool,
    #[serde(default)]
    pub backup: bool,
    pub workers: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct FormatResponseData {
    pub total_files: usize,
    pub formatted_files: usize,
    pub failed_files: usize,
    pub backup_id: Option<String>,
    pub duration_ms: u64,
    pub results: Vec<FileFormatResult>,
}

#[derive(Debug, Serialize)]
pub struct FileFormatResult {
    pub path: PathBuf,
    pub success: bool,
    pub changed: bool,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecoverParams {
    pub backup_id: String,
    pub target: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
pub struct RecoverResponseData {
    pub restored_files: usize,
    pub duration_ms: u64,
}
