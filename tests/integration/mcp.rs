use std::path::PathBuf;
use std::sync::Arc;
use zenith::config::types::{AppConfig, McpConfig, McpUser};
use zenith::mcp::protocol::*;
use zenith::mcp::server::McpServer;
use zenith::storage::cache::HashCache;
use zenith::zeniths::registry::ZenithRegistry;

#[tokio::test]
async fn test_jsonrpc_request_format() {
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        method: "format".to_string(),
        params: Some(serde_json::json!({
            "paths": ["/tmp/test.rs"],
            "recursive": false,
            "backup": true
        })),
    };

    assert_eq!(request.jsonrpc, "2.0");
    assert_eq!(request.method, "format");
    assert!(request.params.is_some());
    assert_eq!(request.id, Some(serde_json::json!(1)));
}

#[tokio::test]
async fn test_jsonrpc_request_recover() {
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(2)),
        method: "recover".to_string(),
        params: Some(serde_json::json!({
            "backup_id": "test-backup-id",
            "target": "/tmp/restore"
        })),
    };

    assert_eq!(request.jsonrpc, "2.0");
    assert_eq!(request.method, "recover");
    assert!(request.params.is_some());
}

#[tokio::test]
async fn test_jsonrpc_response_success() {
    let response: JsonRpcResponse<serde_json::Value> = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        result: Some(serde_json::json!({
            "total_files": 1,
            "formatted_files": 1,
            "failed_files": 0
        })),
        error: None,
    };

    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.result.is_some());
    assert!(response.error.is_none());
}

#[tokio::test]
async fn test_jsonrpc_response_error() {
    let response: JsonRpcResponse<serde_json::Value> = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        result: None,
        error: Some(JsonRpcError {
            code: -32601,
            message: "Method not found".to_string(),
        }),
    };

    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.result.is_none());
    assert!(response.error.is_some());
    assert_eq!(response.error.unwrap().code, -32601);
}

#[tokio::test]
async fn test_format_params_deserialization() {
    let json = serde_json::json!({
        "paths": ["/tmp/test.rs", "/tmp/test.py"],
        "recursive": true,
        "backup": false,
        "workers": 4
    });

    let params: FormatParams = serde_json::from_value(json).unwrap();

    assert_eq!(params.paths.len(), 2);
    assert_eq!(params.paths[0], PathBuf::from("/tmp/test.rs"));
    assert_eq!(params.paths[1], PathBuf::from("/tmp/test.py"));
    assert!(params.recursive);
    assert!(!params.backup);
    assert_eq!(params.workers, Some(4));
}

#[tokio::test]
async fn test_format_params_defaults() {
    let json = serde_json::json!({
        "paths": ["/tmp/test.rs"]
    });

    let params: FormatParams = serde_json::from_value(json).unwrap();

    assert_eq!(params.paths.len(), 1);
    assert!(!params.recursive);
    assert!(!params.backup);
    assert!(params.workers.is_none());
}

#[tokio::test]
async fn test_recover_params_deserialization() {
    let json = serde_json::json!({
        "backup_id": "backup-123",
        "target": "/tmp/restore"
    });

    let params: RecoverParams = serde_json::from_value(json).unwrap();

    assert_eq!(params.backup_id, "backup-123");
    assert_eq!(params.target, Some(PathBuf::from("/tmp/restore")));
}

#[tokio::test]
async fn test_recover_params_without_target() {
    let json = serde_json::json!({
        "backup_id": "backup-123"
    });

    let params: RecoverParams = serde_json::from_value(json).unwrap();

    assert_eq!(params.backup_id, "backup-123");
    assert!(params.target.is_none());
}

#[tokio::test]
async fn test_jsonrpc_error_codes() {
    let errors = vec![
        (-32700, "Parse error"),
        (-32600, "Invalid Request"),
        (-32601, "Method not found"),
        (-32602, "Invalid params"),
        (-32603, "Internal error"),
        (1003, "Format execution error"),
        (1004, "Recovery execution error"),
        (1005, "User context not found"),
        (1006, "Permission denied"),
    ];

    for (code, message) in errors {
        let error = JsonRpcError {
            code,
            message: message.to_string(),
        };
        assert_eq!(error.code, code);
        assert_eq!(error.message, message);
    }
}

#[tokio::test]
async fn test_format_response_data_serialization() {
    let data = FormatResponseData {
        total_files: 10,
        formatted_files: 8,
        failed_files: 2,
        backup_id: Some("backup-123".to_string()),
        duration_ms: 1500,
        results: vec![
            FileFormatResult {
                path: PathBuf::from("/tmp/test1.rs"),
                success: true,
                changed: true,
                error: None,
            },
            FileFormatResult {
                path: PathBuf::from("/tmp/test2.rs"),
                success: false,
                changed: false,
                error: Some("Syntax error".to_string()),
            },
        ],
    };

    let json = serde_json::to_value(&data).unwrap();

    assert_eq!(json["total_files"], 10);
    assert_eq!(json["formatted_files"], 8);
    assert_eq!(json["failed_files"], 2);
    assert_eq!(json["backup_id"], "backup-123");
    assert_eq!(json["duration_ms"], 1500);
    assert_eq!(json["results"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_recover_response_data_serialization() {
    let data = RecoverResponseData {
        restored_files: 5,
        duration_ms: 800,
    };

    let json = serde_json::to_value(&data).unwrap();

    assert_eq!(json["restored_files"], 5);
    assert_eq!(json["duration_ms"], 800);
}

#[tokio::test]
async fn test_file_format_result() {
    let result = FileFormatResult {
        path: PathBuf::from("/tmp/test.rs"),
        success: true,
        changed: true,
        error: None,
    };

    assert_eq!(result.path, PathBuf::from("/tmp/test.rs"));
    assert!(result.success);
    assert!(result.changed);
    assert!(result.error.is_none());
}

#[tokio::test]
async fn test_file_format_result_with_error() {
    let result = FileFormatResult {
        path: PathBuf::from("/tmp/test.rs"),
        success: false,
        changed: false,
        error: Some("Format failed".to_string()),
    };

    assert!(!result.success);
    assert!(!result.changed);
    assert_eq!(result.error, Some("Format failed".to_string()));
}

#[tokio::test]
async fn test_mcp_config() {
    let config = McpConfig {
        enabled: true,
        host: "127.0.0.1".to_string(),
        port: 8080,
        auth_enabled: true,
        api_key: None,
        allowed_origins: vec!["*".to_string()],
        users: vec![
            McpUser {
                api_key: "test-key-1".to_string(),
                role: "admin".to_string(),
            },
            McpUser {
                api_key: "test-key-2".to_string(),
                role: "user".to_string(),
            },
        ],
    };

    assert!(config.enabled);
    assert!(config.auth_enabled);
    assert_eq!(config.users.len(), 2);
    assert_eq!(config.users[0].role, "admin");
    assert_eq!(config.users[1].role, "user");
}

#[tokio::test]
async fn test_mcp_server_creation() {
    let config = AppConfig::default();
    let registry = Arc::new(ZenithRegistry::new());
    let hash_cache = Arc::new(HashCache::new());

    let _server = McpServer::new(config, registry, hash_cache);
}

#[tokio::test]
async fn test_jsonrpc_request_without_id() {
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: None,
        method: "format".to_string(),
        params: None,
    };

    assert_eq!(request.jsonrpc, "2.0");
    assert!(request.id.is_none());
}

#[tokio::test]
async fn test_jsonrpc_request_without_params() {
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        method: "format".to_string(),
        params: None,
    };

    assert_eq!(request.jsonrpc, "2.0");
    assert!(request.params.is_none());
}

#[tokio::test]
async fn test_jsonrpc_invalid_method() {
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        method: "invalid_method".to_string(),
        params: None,
    };

    assert_eq!(request.method, "invalid_method");
}

#[tokio::test]
async fn test_format_params_empty_paths() {
    let json = serde_json::json!({
        "paths": []
    });

    let params: FormatParams = serde_json::from_value(json).unwrap();

    assert!(params.paths.is_empty());
}

#[tokio::test]
async fn test_format_params_multiple_paths() {
    let json = serde_json::json!({
        "paths": ["/tmp/a.rs", "/tmp/b.py", "/tmp/c.js"]
    });

    let params: FormatParams = serde_json::from_value(json).unwrap();

    assert_eq!(params.paths.len(), 3);
}

#[tokio::test]
async fn test_jsonrpc_response_with_null_result() {
    let response: JsonRpcResponse<serde_json::Value> = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        result: Some(serde_json::Value::Null),
        error: None,
    };

    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.result.is_some());
    assert_eq!(response.result.unwrap(), serde_json::Value::Null);
}

#[tokio::test]
async fn test_jsonrpc_error_with_custom_code() {
    let error = JsonRpcError {
        code: 9999,
        message: "Custom error".to_string(),
    };

    assert_eq!(error.code, 9999);
    assert_eq!(error.message, "Custom error");
}

#[tokio::test]
async fn test_mcp_user_roles() {
    let admin = McpUser {
        api_key: "admin-key".to_string(),
        role: "admin".to_string(),
    };

    let user = McpUser {
        api_key: "user-key".to_string(),
        role: "user".to_string(),
    };

    let readonly = McpUser {
        api_key: "readonly-key".to_string(),
        role: "readonly".to_string(),
    };

    assert_eq!(admin.role, "admin");
    assert_eq!(user.role, "user");
    assert_eq!(readonly.role, "readonly");
}

#[tokio::test]
async fn test_format_response_data_without_backup() {
    let data = FormatResponseData {
        total_files: 5,
        formatted_files: 5,
        failed_files: 0,
        backup_id: None,
        duration_ms: 1000,
        results: vec![],
    };

    assert!(data.backup_id.is_none());
    assert_eq!(data.total_files, 5);
    assert_eq!(data.formatted_files, 5);
}

#[tokio::test]
async fn test_recover_params_empty_backup_id() {
    let json = serde_json::json!({
        "backup_id": ""
    });

    let params: RecoverParams = serde_json::from_value(json).unwrap();

    assert_eq!(params.backup_id, "");
}

#[tokio::test]
async fn test_jsonrpc_version_validation() {
    let valid_versions = vec!["2.0"];

    for version in valid_versions {
        let request = JsonRpcRequest {
            jsonrpc: version.to_string(),
            id: Some(serde_json::json!(1)),
            method: "format".to_string(),
            params: None,
        };
        assert_eq!(request.jsonrpc, version);
    }
}
