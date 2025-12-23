use crate::config::types::AppConfig;
use crate::mcp::protocol::*;
use crate::services::formatter::ZenithService;
use crate::storage::backup::BackupService;
use crate::zeniths::registry::ZenithRegistry;
use axum::{routing::post, Json, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct McpServer {
    config: AppConfig,
    registry: Arc<ZenithRegistry>,
}

impl McpServer {
    pub fn new(config: AppConfig, registry: Arc<ZenithRegistry>) -> Self {
        Self { config, registry }
    }

    pub async fn run(&self, addr: SocketAddr) -> crate::error::Result<()> {
        let app_state = Arc::new(AppState {
            config: self.config.clone(),
            registry: self.registry.clone(),
        });

        let app = Router::new()
            .route("/", post(handle_json_rpc))
            .with_state(app_state);

        info!("MCP Server listening on {}", addr);
        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

struct AppState {
    config: AppConfig,
    registry: Arc<ZenithRegistry>,
}

async fn handle_json_rpc(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(req): Json<JsonRpcRequest>,
) -> Json<JsonRpcResponse<serde_json::Value>> {
    let response = match req.method.as_str() {
        "format" => handle_format(state, req.params).await,
        "recover" => handle_recover(state, req.params).await,
        _ => Err(JsonRpcError {
            code: -32601,
            message: "Method not found".into(),
        }),
    };

    match response {
        Ok(result) => Json(JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: Some(result),
            error: None,
        }),
        Err(err) => Json(JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: None,
            error: Some(err),
        }),
    }
}

async fn handle_format(
    state: Arc<AppState>,
    params: Option<serde_json::Value>,
) -> Result<serde_json::Value, JsonRpcError> {
    let params: FormatParams = serde_json::from_value(params.unwrap_or(serde_json::Value::Null))
        .map_err(|_| JsonRpcError {
            code: -32602,
            message: "Invalid params".into(),
        })?;

    let mut config = state.config.clone();
    config.global.recursive = params.recursive;
    config.global.backup_enabled = params.backup;
    if let Some(w) = params.workers {
        config.concurrency.workers = w;
    }

    let backup_service = Arc::new(BackupService::new(config.backup.clone()));
    let service = ZenithService::new(
        config,
        state.registry.clone(),
        backup_service.clone(),
        false,
    );

    let start = std::time::Instant::now();
    let results = service
        .format_paths(params.paths)
        .await
        .map_err(|e| JsonRpcError {
            code: 1003,
            message: e.to_string(),
        })?;
    let duration = start.elapsed().as_millis() as u64;

    let total = results.len();
    let success = results.iter().filter(|r| r.success).count();
    let failed = total - success;

    let response = FormatResponseData {
        total_files: total,
        formatted_files: success,
        failed_files: failed,
        backup_id: Some(backup_service.get_session_id().to_string()),
        duration_ms: duration,
        results: results
            .into_iter()
            .map(|r| FileFormatResult {
                path: r.file_path,
                success: r.success,
                changed: r.changed,
                error: r.error,
            })
            .collect(),
    };

    serde_json::to_value(response).map_err(|_| JsonRpcError {
        code: -32603,
        message: "Serialization error".into(),
    })
}

async fn handle_recover(
    state: Arc<AppState>,
    params: Option<serde_json::Value>,
) -> Result<serde_json::Value, JsonRpcError> {
    let params: RecoverParams = serde_json::from_value(params.unwrap_or(serde_json::Value::Null))
        .map_err(|_| JsonRpcError {
        code: -32602,
        message: "Invalid params".into(),
    })?;

    let backup_service = BackupService::new(state.config.backup.clone());

    let start = std::time::Instant::now();
    let count = backup_service
        .recover(&params.backup_id, params.target)
        .await
        .map_err(|e| JsonRpcError {
            code: 1004,
            message: e.to_string(),
        })?;
    let duration = start.elapsed().as_millis() as u64;

    let response = RecoverResponseData {
        restored_files: count,
        duration_ms: duration,
    };

    serde_json::to_value(response).map_err(|_| JsonRpcError {
        code: -32603,
        message: "Serialization error".into(),
    })
}
