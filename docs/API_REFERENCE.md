<div align="center">

# 📘 API Reference

### Complete API Documentation

[🏠 Home](../README.md) • [📖 User Guide](USER_GUIDE.md) • [🏗️ Architecture](ARCHITECTURE.md)

---

</div>

## 📋 Table of Contents

- [Overview](#overview)
- [CLI API](#cli-api)
  - [Commands](#commands)
  - [Arguments](#arguments)
- [Configuration API](#configuration-api)
  - [AppConfig](#appconfig)
  - [GlobalConfig](#globalconfig)
  - [ZenithSettings](#zenithsettings)
  - [BackupConfig](#backupconfig)
  - [ConcurrencyConfig](#concurrencyconfig)
  - [LimitsConfig](#limitsconfig)
  - [McpConfig](#mcpconfig)
  - [SecurityConfig](#securityconfig)
- [Core API](#core-api)
  - [Zenith Trait](#zenith-trait)
  - [Formatter Service](#formatter-service)
- [Storage API](#storage-api)
  - [BackupService](#backupservice)
  - [HashCache](#hashcache)
- [MCP Server API](#mcp-server-api)
- [Plugin API](#plugin-api)
- [Error Handling](#error-handling)
- [Examples](#examples)

---

## Overview

<div align="center">

### 🎯 API Design Principles

</div>

<table>
<tr>
<td width="25%" align="center">
<img src="https://img.icons8.com/fluency/96/000000/easy.png" width="64"><br>
<b>Simple</b><br>
Intuitive and easy to use
</td>
<td width="25%" align="center">
<img src="https://img.icons8.com/fluency/96/000000/security-checked.png" width="64"><br>
<b>Safe</b><br>
Type-safe and secure by default
</td>
<td width="25%" align="center">
<img src="https://img.icons8.com/fluency/96/000000/module.png" width="64"><br>
<b>Composable</b><br>
Build complex workflows easily
</td>
<td width="25%" align="center">
<img src="https://img.icons8.com/fluency/96/000000/documentation.png" width="64"><br>
<b>Well-documented</b><br>
Comprehensive documentation
</td>
</tr>
</table>

Zenith provides a comprehensive API for code formatting, backup management, and MCP server integration. The API is designed to be type-safe, composable, and easy to use.

---

## CLI API

### Commands

<div align="center">

#### 🚀 Available Commands

</div>

---

#### `format`

Format files or directories.

<table>
<tr>
<td width="30%"><b>Command</b></td>
<td width="70%">

```bash
zenith format <PATHS>... [OPTIONS]
```

</td>
</tr>
<tr>
<td><b>Description</b></td>
<td>Format the specified files or directories using configured formatters.</td>
</tr>
<tr>
<td><b>Arguments</b></td>
<td>

- `PATHS...` - One or more file/directory paths to format (required)

</td>
</tr>
</table>

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `-r, --recursive` | bool | false | Recursively process subdirectories |
| `--no-backup` | bool | false | Disable automatic backup before formatting |
| `-w, --workers` | usize | CPU count | Number of concurrent worker threads |
| `--check` | bool | false | Dry-run mode, don't modify files |
| `--watch` | bool | false | Enable file watching mode for real-time formatting |

**Example:**

```bash
# Format single file
zenith format src/main.rs

# Format directory recursively with backup
zenith format src/ --recursive --workers=4

# Check without modifying (dry-run)
zenith format . --check --recursive

# Watch mode - monitor files and format automatically
zenith format src/ --watch
```

---

#### `doctor`

Check system environment.

<table>
<tr>
<td width="30%"><b>Command</b></td>
<td width="70%">

```bash
zenith doctor [OPTIONS]
```

</td>
</tr>
<tr>
<td><b>Description</b></td>
<td>Verify system environment and dependencies for Zenith.</td>
</tr>
</table>

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `-v, --verbose` | bool | false | Show detailed diagnostic information |

**Example:**

```bash
zenith doctor
zenith doctor --verbose
```

---

#### `list-backups`

List all available backups.

<table>
<tr>
<td width="30%"><b>Command</b></td>
<td width="70%">

```bash
zenith list-backups
```

</td>
</tr>
<tr>
<td><b>Description</b></td>
<td>Display all existing backups with their creation time and size.</td>
</tr>
</table>

**Example:**

```bash
zenith list-backups
```

---

#### `recover`

Restore files from a backup.

<table>
<tr>
<td width="30%"><b>Command</b></td>
<td width="70%">

```bash
zenith recover <BACKUP_ID> [OPTIONS]
```

</td>
</tr>
<tr>
<td><b>Arguments</b></td>
<td>

- `BACKUP_ID` - The backup session ID to restore (required)

</td>
</tr>
</table>

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `-t, --target` | Path | Current dir | Target directory for restoration |

**Example:**

```bash
zenith recover backup_20251226_143022
zenith recover backup_20251226_143022 --target ./restored/
```

---

#### `clean-backups`

Remove old backups.

<table>
<tr>
<td width="30%"><b>Command</b></td>
<td width="70%">

```bash
zenith clean-backups [OPTIONS]
```

</td>
</tr>
</table>

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `-d, --days` | u32 | 7 | Number of days to retain backups |

**Example:**

```bash
# Clean backups older than 7 days
zenith clean-backups

# Clean backups older than 30 days
zenith clean-backups --days=30
```

---

#### `mcp`

Start the MCP (Model Context Protocol) server.

<table>
<tr>
<td width="30%"><b>Command</b></td>
<td width="70%">

```bash
zenith mcp [OPTIONS]
```

</td>
</tr>
</table>

**Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `-a, --addr` | String | 127.0.0.1:9000 | Server listen address |

**Example:**

```bash
# Start MCP server with default address
zenith mcp

# Start on custom address
zenith mcp --addr=0.0.0.0:9000
```

---

#### `auto-rollback`

Automatically rollback to the latest backup.

<table>
<tr>
<td width="30%"><b>Command</b></td>
<td width="70%">

```bash
zenith auto-rollback
```

</td>
</tr>
<tr>
<td><b>Description</b></td>
<td>Revert all changes to the most recent backup state.</td>
</tr>
</table>

**Example:**

```bash
zenith auto-rollback
```

---

### Arguments

<div align="center">

#### ⚙️ Global CLI Arguments

</div>

#### `Cli` Struct

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct Cli {
    pub command: Commands,
    pub config: Option<PathBuf>,
    pub log_level: String,
}
```

</td>
</tr>
</table>

**Global Options:**

| Option | Env Variable | Default | Description |
|--------|--------------|---------|-------------|
| `-c, --config` | ZENITH_CONFIG | None | Custom config file path |
| `-L, --log-level` | ZENITH_LOG_LEVEL | info | Log level (debug, info, warn, error) |

---

## Configuration API

### AppConfig

<div align="center">

#### 📦 Main Configuration Container

</div>

---

#### `AppConfig`

The root configuration structure for Zenith.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct AppConfig {
    pub global: GlobalConfig,
    pub zeniths: HashMap<String, ZenithSettings>,
    pub backup: BackupConfig,
    pub concurrency: ConcurrencyConfig,
    pub limits: LimitsConfig,
    pub mcp: McpConfig,
    pub security: SecurityConfig,
}
```

</td>
</tr>
</table>

---

### GlobalConfig

#### `GlobalConfig`

Global settings that apply to all operations.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct GlobalConfig {
    pub backup_enabled: bool,
    pub log_level: String,
    pub recursive: bool,
    pub cache_enabled: bool,
    pub config_dir: String,
}
```

</td>
</tr>
</table>

**Fields:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `backup_enabled` | bool | true | Enable automatic backup before formatting |
| `log_level` | String | "info" | Logging verbosity level |
| `recursive` | bool | true | Recursively process directories |
| `cache_enabled` | bool | true | Enable hash-based caching |
| `config_dir` | String | ".zenith" | Config and plugin directory |

**Example:**

```toml
[global]
backup_enabled = true
log_level = "debug"
recursive = true
cache_enabled = true
config_dir = ".zenith"
```

---

### ZenithSettings

#### `ZenithSettings`

Configuration for individual formatter tools.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct ZenithSettings {
    pub enabled: bool,
    pub config_path: Option<String>,
    pub use_default: bool,
}
```

</td>
</tr>
</table>

**Fields:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | bool | true | Enable this formatter |
| `config_path` | Option<String> | None | Path to formatter config file |
| `use_default` | bool | true | Use default formatting rules |

**Example:**

```toml
[zeniths.rust]
enabled = true
config_path = ".rustfmt.toml"
use_default = true

[zeniths.python]
enabled = true
config_path = "pyproject.toml"
use_default = true

[zeniths.markdown]
enabled = true
use_default = true
```

**Supported Formatters:**

| Formatter | Extensions | Config File |
|-----------|------------|-------------|
| Rust | .rs | .rustfmt.toml |
| Python | .py | pyproject.toml |
| Markdown | .md | None |
| JSON | .json | None |
| TOML | .toml | None |
| YAML | .yaml, .yml | None |
| Shell | .sh, .bash | None |
| C/C++ | .c, .cpp, .h | None |
| Java | .java | None |
| INI | .ini | None |
| Go | .go | None |
| JavaScript | .js | .prettierrc |
| TypeScript | .ts | .prettierrc |

---

### BackupConfig

#### `BackupConfig`

Configuration for backup operations.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct BackupConfig {
    pub dir: String,
    pub retention_days: u32,
}
```

</td>
</tr>
</table>

**Fields:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dir` | String | ".zenith_backup" | Backup directory path |
| `retention_days` | u32 | 7 | Number of days to retain backups |

**Example:**

```toml
[backup]
dir = ".zenith_backups"
retention_days = 30
```

---

### ConcurrencyConfig

#### `ConcurrencyConfig`

Performance and concurrency settings.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct ConcurrencyConfig {
    pub workers: usize,
    pub batch_size: usize,
}
```

</td>
</tr>
</table>

**Fields:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `workers` | usize | CPU cores | Number of formatting worker threads |
| `batch_size` | usize | 100 | Number of files to process in batch |

**Example:**

```toml
[concurrency]
workers = 8
batch_size = 200
```

---

### LimitsConfig

#### `LimitsConfig`

Resource and file limit settings.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct LimitsConfig {
    pub max_file_size_mb: u64,
    pub max_memory_mb: u64,
}
```

</td>
</tr>
</table>

**Fields:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_file_size_mb` | u64 | 10 | Maximum file size in MB |
| `max_memory_mb` | u64 | 100 | Maximum memory usage in MB |

**Example:**

```toml
[limits]
max_file_size_mb = 50
max_memory_mb = 500
```

---

### SecurityConfig

#### `SecurityConfig`

Plugin security settings.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct SecurityConfig {
    pub allowed_plugin_commands: Vec<String>,
    pub allow_absolute_paths: bool,
    pub allow_relative_paths: bool,
}
```

</td>
</tr>
</table>

**Fields:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `allowed_plugin_commands` | Vec<String> | [] | Whitelist of allowed plugin commands |
| `allow_absolute_paths` | bool | true | Allow plugins to use absolute paths |
| `allow_relative_paths` | bool | false | Allow plugins to use relative paths |

**Example:**

```toml
[security]
allowed_plugin_commands = ["fmt", "lint"]
allow_absolute_paths = true
allow_relative_paths = false
```

---

### McpConfig

#### `McpConfig`

MCP server configuration.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct McpConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub auth_enabled: bool,
    pub api_key: Option<String>,
    pub allowed_origins: Vec<String>,
    pub users: Vec<McpUser>,
}
```

</td>
</tr>
</table>

**Fields:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | bool | false | Enable MCP server |
| `host` | String | "127.0.0.1" | Server host |
| `port` | u16 | 8080 | Server port |
| `auth_enabled` | bool | true | Enable API key authentication |
| `api_key` | Option<String> | None | Main API key |
| `allowed_origins` | Vec<String> | ["*"] | Allowed CORS origins |
| `users` | Vec<McpUser> | [] | User list with roles |

**Example:**

```toml
[mcp]
enabled = true
host = "127.0.0.1"
port = 9000
auth_enabled = true
api_key = "your-api-key-here"
allowed_origins = ["http://localhost:3000"]

[[mcp.users]]
api_key = "user1-key"
role = "user"

[[mcp.users]]
api_key = "admin-key"
role = "admin"
```

**McpUser:**

```rust
pub struct McpUser {
    pub api_key: String,
    pub role: String,
}
```

---

## Core API

### Zenith Trait

<div align="center">

#### 🎯 Core Trait for Formatters

</div>

---

#### `Zenith`

The core trait that all formatters implement.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
#[async_trait]
pub trait Zenith: Send + Sync {
    fn name(&self) -> &str;
    fn extensions(&self) -> &[&str];
    fn priority(&self) -> i32;
    async fn format(&self, content: &[u8], path: &Path, config: &ZenithConfig) -> Result<Vec<u8>>;
    async fn validate(&self, content: &[u8]) -> Result<bool>;
}
```

</td>
</tr>
</table>

**Required Methods:**

| Method | Returns | Description |
|--------|---------|-------------|
| `name()` | &str | Unique name identifier for the formatter |
| `extensions()` | &[&str] | Supported file extensions |
| `priority()` | i32 | Execution priority (higher = executed first) |

**Optional Methods:**

| Method | Returns | Default | Description |
|--------|---------|---------|-------------|
| `format()` | Result<Vec<u8>> | - | Format file content |
| `validate()` | Result<bool> | true | Validate formatted content |

**Implementation Example:**

```rust
#[async_trait]
impl Zenith for RustZenith {
    fn name(&self) -> &str {
        "rustfmt"
    }

    fn extensions(&self) -> &[&str] {
        &["rs"]
    }

    fn priority(&self) -> i32 {
        100
    }

    async fn format(&self, content: &[u8], path: &Path, config: &ZenithConfig) -> Result<Vec<u8>> {
        // Format implementation
        Ok(formatted_content)
    }
}
```

---

### Formatter Service

#### `ZenithService`

The main service for executing formatting operations.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct ZenithService {
    registry: ZenithRegistry,
    cache: HashCache,
    backup: BackupService,
    config: AppConfig,
}
```

</td>
</tr>
</table>

**Methods:**

| Method | Returns | Description |
|--------|---------|-------------|
| `new(config: AppConfig)` | Self | Create a new service instance |
| `format_file(path: &Path)` | Result<()> | Format a single file |
| `format_directory(path: &Path, recursive: bool)` | Result<()> | Format all files in directory |
| `check(path: &Path)` | Result<bool> | Check if files need formatting |

**Example:**

```rust
use zenith::{ZenithService, AppConfig};

let config = AppConfig::load("zenith.toml")?;
let service = ZenithService::new(config);

// Format a file
service.format_file(Path::new("src/main.rs"))?;

// Check formatting (dry-run)
let needs_formatting = service.check(Path::new("src/main.rs"))?;
```

---

## Storage API

### BackupService

<div align="center">

#### 💾 Backup Management

</div>

---

#### `BackupService`

Service for creating and managing backups.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct BackupService {
    config: BackupConfig,
    session_id: String,
}
```

</td>
</tr>
</table>

**Methods:**

| Method | Returns | Description |
|--------|---------|-------------|
| `new(config: BackupConfig)` | Self | Create a new backup service |
| `init()` | Result<()> | Initialize backup directory |
| `backup_file(root, path, content)` | Result<()> | Backup a single file |
| `list_backups()` | Result<Vec<(String, SystemTime, u64)>> | List all backups |
| `restore(backup_id, target)` | Result<()> | Restore from backup |
| `clean_older_than(days: u32)` | Result<()> | Remove old backups |
| `get_session_id()` | &str | Get current session ID |

**Example:**

```rust
use zenith::storage::BackupService;

let backup = BackupService::new(config.backup.clone());
backup.init().await?;

// Backup a file
backup.backup_file(&root_path, &file_path, &content).await?;

// List backups
let backups = backup.list_backups().await?;
for (id, time, size) in backups {
    println!("{}: {} bytes - {:?}", id, size, time);
}

// Restore from backup
backup.restore("backup_20251226_143022", &target).await?;

// Clean old backups
backup.clean_older_than(30).await?;
```

<details>
<summary><b>📝 Notes</b></summary>

- Backups include BLAKE3 hash verification
- Directory structure is preserved
- Each backup session has a unique timestamped ID
- Backups are stored in the configured backup directory

</details>

---

### HashCache

#### `HashCache`

Hash-based content cache for optimization.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct HashCache {
    // internal fields
}
```

</td>
</tr>
</table>

**Methods:**

| Method | Returns | Description |
|--------|---------|-------------|
| `new()` | Self | Create a new cache |
| `with_cache_dir(path)` | Self | Create cache with persistence directory |
| `with_config_aware(bool)` | Self | Enable config-aware caching |
| `with_max_entry_age(duration)` | Self | Set cache entry expiration |
| `compute_file_state(path)` | Result<FileState> | Compute file hash and metadata |
| `needs_processing(path)` | Result<bool> | Check if file needs formatting |
| `needs_processing_with_config(path, config)` | Result<bool> | Check with config awareness |
| `update(path, state)` | Result<()> | Update cache entry |
| `update_with_config(path, config)` | Result<> | Update with config hash |
| `remove(path)` | Result<> | Remove cached entry |
| `clear()` | Result<()> | Clear all cached entries |
| `is_cached(path)` | bool | Check if file is in cache |
| `get_cached_state(path)` | Option<FileState> | Get cached file state |
| `cleanup()` | Result<usize> | Remove expired cache entries |
| `stats()` | CacheStats | Get cache statistics |
| `batch_needs_processing(paths)` | Result<Vec<bool>> | Batch check files |
| `invalidate_matching(predicate)` | Result<usize> | Invalidate entries matching predicate |
| `save()` | Result<()> | Persist cache to disk |
| `save_background()` | () | Save cache asynchronously |
| `load()` | Result<()> | Load cache from disk |

**Example:**

```rust
use zenith::storage::HashCache;
use std::time::Duration;

let cache = HashCache::new()
    .with_cache_dir(".zenith_cache".into())
    .with_config_aware(true)
    .with_max_entry_age(Duration::from_secs(3600));

// Check if file needs formatting
if cache.needs_processing(path).await? {
    // File changed or not cached, format it
    let formatted = formatter.format(&current_content).await?;

    // Update cache
    cache.update_with_config(path, &config).await?;
}
```

---

## MCP Server API

### McpServer

<div align="center">

#### 🌐 MCP Protocol Server

</div>

---

#### `McpServer`

Model Context Protocol server for AI integration.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct McpServer {
    config: AppConfig,
    registry: Arc<ZenithRegistry>,
    hash_cache: Arc<HashCache>,
}
```

</td>
</tr>
</table>

**Methods:**

| Method | Returns | Description |
|--------|---------|-------------|
| `new(config, registry, cache)` | Self | Create a new MCP server |
| `run(addr: SocketAddr)` | Result<()> | Start the server |

**Example:**

```rust
use zenith::McpServer;
use std::net::SocketAddr;

let server = McpServer::new(
    config,
    registry,
    cache,
);

let addr = SocketAddr::new("127.0.0.1".parse()?, 9000);
server.run(addr).await?;
```

**Protocol Endpoints:**

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | POST | JSON-RPC request handler |

**Authentication:**

When `auth_enabled` is true, requests require `Authorization` header:

```bash
curl -X POST http://127.0.0.1:9000/ \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{"jsonrpc": "2.0", "method": "format", "params": {...}, "id": 1}'
```

---

## Plugin API

### Plugin System

<div align="center">

#### 🔌 Extensibility Framework

</div>

---

#### `PluginLoader`

Manages dynamic loading of custom formatters.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct PluginLoader {
    // internal fields
}
```

</td>
</tr>
</table>

**Methods:**

| Method | Returns | Description |
|--------|---------|-------------|
| `new(config_dir: &Path)` | Self | Create a new plugin loader |
| `load_plugins()` | Result<Vec<Box<dyn Zenith>>> | Load all plugins |
| `load_plugin(path: &Path)` | Result<Box<dyn Zenith>> | Load a single plugin |
| `unload_plugin(name: &str)` | Result<()> | Unload a plugin |

---

#### `PluginInfo`

Information about a loaded plugin.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct PluginInfo {
    pub name: String,
    pub extensions: Vec<String>,
    pub version: String,
}
```

</td>
</tr>
</table>

---

#### `PluginConfig`

Configuration for plugin loading.

<table>
<tr>
<td width="30%"><b>Type</b></td>
<td width="70%">

```rust
pub struct PluginConfig {
    pub enabled: bool,
    pub path: String,
    pub auto_update: bool,
}
```

</td>
</tr>
</table>

**Example:**

```toml
[plugins.my_formatter]
enabled = true
path = "./plugins/my_formatter.so"
auto_update = false
```

---

## Error Handling

<div align="center">

#### 🚨 Error Types and Handling

</div>

### `ZenithError` Enum

```rust
pub enum ZenithError {
    // IO Errors
    IoError(std::io::Error),
    FileNotFound(String),
    PermissionDenied(String),
    
    // Formatting Errors
    FormatFailed(String),
    NoFormatterFound(String),
    
    // Backup Errors
    BackupFailed(String),
    RestoreFailed(String),
    BackupNotFound(String),
    
    // Configuration Errors
    ConfigParseError(String),
    ConfigValidationError(String),
    
    // MCP Errors
    McpServerError(String),
    AuthenticationFailed,
    
    // Plugin Errors
    PluginLoadError(String),
    PluginNotFound(String),
    
    // Common Errors
    NotFound(String),
    InvalidInput(String),
    Timeout,
}
```

### Error Handling Pattern

<table>
<tr>
<td width="50%">

**Pattern Matching**
```rust
match service.format_file(path) {
    Ok(_) => {
        println!("✅ File formatted successfully");
    }
    Err(ZenithError::NoFormatterFound(ext)) => {
        eprintln!("No formatter for: {}", ext);
    }
    Err(ZenithError::BackupFailed(msg)) => {
        eprintln!("Backup failed: {}", msg);
    }
    Err(e) => {
        eprintln!("Error: {:?}", e);
    }
}
```

</td>
<td width="50%">

**? Operator**
```rust
fn process_files() -> Result<(), ZenithError> {
    let config = AppConfig::load("zenith.toml")?;
    let service = ZenithService::new(config)?;
    
    service.format_file(Path::new("src/main.rs"))?;
    service.format_directory(Path::new("src"), true)?;
    
    Ok(())
}
```

</td>
</tr>
</table>

---

## Examples

<div align="center">

### 💡 Common Usage Patterns

</div>

### Example 1: Basic File Formatting

```rust
use zenith::{ZenithService, AppConfig};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AppConfig::load("zenith.toml")?;
    
    // Create service
    let service = ZenithService::new(config);
    
    // Format a single file
    service.format_file(Path::new("src/main.rs"))?;
    
    println!("✅ File formatted successfully!");
    
    Ok(())
}
```

### Example 2: Batch Directory Formatting

```rust
use zenith::ZenithService;
use std::path::Path;

async fn format_project() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load("zenith.toml")?;
    let service = ZenithService::new(config);
    
    // Format entire project
    service.format_directory(Path::new("src"), true)?;
    
    // Check formatting status
    let needs_formatting = service.check(Path::new("src"))?;
    
    if needs_formatting {
        println!("⚠️ Some files need formatting");
    } else {
        println!("✅ All files are properly formatted");
    }
    
    Ok(())
}
```

### Example 3: Custom Configuration

```rust
use zenith::config::types::{AppConfig, GlobalConfig, BackupConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig {
        global: GlobalConfig {
            backup_enabled: true,
            log_level: "debug".into(),
            recursive: true,
            cache_enabled: true,
            config_dir: ".zenith".into(),
        },
        backup: BackupConfig {
            dir: ".backups".into(),
            max_backups: 30,
            compress: true,
        },
        ..Default::default()
    };
    
    // Use custom config...
    
    Ok(())
}
```

### Example 4: MCP Server Setup

```rust
use zenith::{McpServer, AppConfig};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load("zenith.toml")?;
    
    // Configure MCP
    let mcp_config = config.mcp.clone();
    
    let server = McpServer::new(
        config,
        Arc::new(registry),
        Arc::new(cache),
    );
    
    let addr = SocketAddr::new(
        mcp_config.host.parse()?,
        mcp_config.port,
    );
    
    println!("Starting MCP server on {}", addr);
    server.run(addr).await?;
    
    Ok(())
}
```

### Example 5: Plugin Management

```rust
use zenith::plugins::PluginLoader;
use std::path::Path;

async fn load_custom_plugins() -> Result<(), Box<dyn std::error::Error>> {
    let plugin_dir = Path::new("./plugins");
    let loader = PluginLoader::new(plugin_dir);
    
    // Load all plugins
    let plugins = loader.load_plugins()?;
    
    for plugin in &plugins {
        println!("Loaded plugin: {}", plugin.name());
        println!("  Extensions: {:?}", plugin.extensions());
    }
    
    Ok(())
}
```

---

<div align="center">

**[📖 User Guide](USER_GUIDE.md)** • **[🏗️ Architecture](ARCHITECTURE.md)** • **[🏠 Home](../README.md)**

Made with ❤️ by the Zenith Team

[⬆ Back to Top](#-api-reference)

</div>
