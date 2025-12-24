use crate::config::types::AppConfig;
use crate::mcp::protocol::*;
use crate::services::formatter::ZenithService;
use crate::storage::backup::BackupService;
use crate::storage::cache::HashCache;
use crate::zeniths::registry::ZenithRegistry;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, warn};

pub struct McpServer {
    config: AppConfig,
    registry: Arc<ZenithRegistry>,
    hash_cache: Arc<HashCache>,
}

impl McpServer {
    pub fn new(
        config: AppConfig,
        registry: Arc<ZenithRegistry>,
        hash_cache: Arc<HashCache>,
    ) -> Self {
        Self {
            config,
            registry,
            hash_cache,
        }
    }

    pub async fn run(&self, addr: SocketAddr) -> crate::error::Result<()> {
        let app_state = Arc::new(AppState {
            config: self.config.clone(),
            registry: self.registry.clone(),
            hash_cache: self.hash_cache.clone(),
        });

        let app = Router::new()
            .route("/", post(handle_json_rpc))
            .route_layer(axum::middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            ))
            .with_state(app_state);

        info!(
            "MCP Server listening on {} (auth: {})",
            addr, self.config.mcp.auth_enabled
        );
        let listener = TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

struct AppState {
    config: AppConfig,
    registry: Arc<ZenithRegistry>,
    hash_cache: Arc<HashCache>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct UserContext {
    api_key: String,
    role: String,
}

async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    if !state.config.mcp.auth_enabled {
        let user_context = UserContext {
            api_key: "default".into(),
            role: "admin".into(),
        };
        request.extensions_mut().insert(user_context);
        return Ok(next.run(request).await);
    }

    let auth_header = headers.get("authorization");

    match auth_header {
        Some(header_value) => {
            let header_str = header_value.to_str().unwrap_or("");

            if let Some(token) = header_str.strip_prefix("Bearer ") {
                for user in &state.config.mcp.users {
                    if user.api_key == token {
                        let user_context = UserContext {
                            api_key: user.api_key.clone(),
                            role: user.role.clone(),
                        };
                        request.extensions_mut().insert(user_context);
                        return Ok(next.run(request).await);
                    }
                }

                warn!("Invalid or unknown authorization token in request");
                return Err(StatusCode::UNAUTHORIZED);
            }

            warn!("Invalid authorization header format");
            Err(StatusCode::UNAUTHORIZED)
        }
        None => {
            warn!("Missing authorization header");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

fn check_method_permission(method: &str, role: &str) -> bool {
    match role {
        "admin" => true,
        "user" => matches!(method, "format" | "recover"),
        "readonly" => method == "format",
        _ => false,
    }
}

async fn handle_json_rpc(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    user_context: Option<axum::Extension<UserContext>>,
    Json(req): Json<JsonRpcRequest>,
) -> Json<JsonRpcResponse<serde_json::Value>> {
    let user_context = match user_context {
        Some(ctx) => ctx.0,
        None => {
            return Json(JsonRpcResponse {
                jsonrpc: "2.0".into(),
                id: req.id,
                result: None,
                error: Some(JsonRpcError {
                    code: 1005,
                    message: "User context not found".into(),
                }),
            });
        }
    };

    if !check_method_permission(&req.method, &user_context.role) {
        return Json(JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: None,
            error: Some(JsonRpcError {
                code: 1006,
                message: format!(
                    "Permission denied for method '{}' with role '{}'",
                    req.method, user_context.role
                ),
            }),
        });
    }

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
        state.hash_cache.clone(),
        false,
    );

    let start = std::time::Instant::now();
    let string_paths: Vec<String> = params
        .paths
        .into_iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    let results = service
        .format_paths(string_paths)
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
