# Zenith 配置文档

## 概述

Zenith 使用 JSON 或 TOML 格式的配置文件来控制格式化行为。配置文件默认位于项目根目录的 `.zenith.json` 或 `.zenith.toml`，也可以通过 `--config` 参数指定。

## 配置结构

### 完整配置示例

```json
{
  "global": {
    "backup_enabled": true,
    "log_level": "info",
    "recursive": true,
    "cache_enabled": true,
    "config_dir": ".zenith"
  },
  "zeniths": {
    "rust": {
      "enabled": true,
      "config_path": "rustfmt.toml",
      "use_default": true
    },
    "python": {
      "enabled": true,
      "config_path": "ruff.toml",
      "use_default": true
    }
  },
  "backup": {
    "dir": ".zenith_backup",
    "retention_days": 7
  },
  "concurrency": {
    "workers": 4,
    "batch_size": 100
  },
  "limits": {
    "max_file_size_mb": 10,
    "max_memory_mb": 100
  },
  "mcp": {
    "enabled": false,
    "host": "127.0.0.1",
    "port": 8080,
    "auth_enabled": true,
    "allowed_origins": ["*"],
    "users": []
  }
}
```

## 全局配置

### global

控制 Zenith 的全局行为。

| 字段 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `backup_enabled` | boolean | `true` | 是否启用备份功能 |
| `log_level` | string | `"info"` | 日志级别：`debug`, `info`, `warn`, `error` |
| `recursive` | boolean | `true` | 是否递归处理子目录 |
| `cache_enabled` | boolean | `true` | 是否启用文件哈希缓存 |
| `config_dir` | string | `".zenith"` | 配置文件目录 |

### 示例

```json
{
  "global": {
    "backup_enabled": true,
    "log_level": "info",
    "recursive": true,
    "cache_enabled": true,
    "config_dir": ".zenith"
  }
}
```

## 格式化器配置

### zeniths

为不同语言的格式化器配置特定设置。每个格式化器可以通过文件扩展名或名称来引用。

| 字段 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `enabled` | boolean | `true` | 是否启用该格式化器 |
| `config_path` | string | `null` | 自定义配置文件路径 |
| `use_default` | boolean | `true` | 是否使用默认规则 |

### 支持的格式化器

#### Rust

```json
{
  "zeniths": {
    "rust": {
      "enabled": true,
      "config_path": "rustfmt.toml",
      "use_default": true
    }
  }
}
```

#### Python

```json
{
  "zeniths": {
    "python": {
      "enabled": true,
      "config_path": "ruff.toml",
      "use_default": true
    }
  }
}
```

#### JavaScript/TypeScript (Prettier)

```json
{
  "zeniths": {
    "prettier": {
      "enabled": true,
      "config_path": ".prettierrc",
      "use_default": true
    }
  }
}
```

#### 其他语言

支持的格式化器包括：
- `c` - C 语言
- `java` - Java
- `shell` - Shell 脚本
- `toml` - TOML 配置文件
- `ini` - INI 配置文件

## 备份配置

### backup

控制文件备份行为。

| 字段 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `dir` | string | `".zenith_backup"` | 备份目录路径 |
| `retention_days` | number | `7` | 备份保留天数 |

### 示例

```json
{
  "backup": {
    "dir": ".zenith_backup",
    "retention_days": 7
  }
}
```

## 并发配置

### concurrency

控制文件处理的并发行为。

| 字段 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `workers` | number | CPU 核心数 | 并发工作线程数 |
| `batch_size` | number | `100` | 批处理大小 |

### 示例

```json
{
  "concurrency": {
    "workers": 4,
    "batch_size": 100
  }
}
```

## 限制配置

### limits

设置资源使用限制。

| 字段 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `max_file_size_mb` | number | `10` | 单个文件最大大小（MB） |
| `max_memory_mb` | number | `100` | 最大内存使用（MB） |

### 示例

```json
{
  "limits": {
    "max_file_size_mb": 10,
    "max_memory_mb": 100
  }
}
```

## MCP 服务器配置

### mcp

配置 MCP (Model Context Protocol) 服务器。

| 字段 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `enabled` | boolean | `false` | 是否启用 MCP 服务器 |
| `host` | string | `"127.0.0.1"` | 服务器监听地址 |
| `port` | number | `8080` | 服务器监听端口 |
| `auth_enabled` | boolean | `true` | 是否启用身份验证 |
| `api_key` | string | `null` | API 密钥 |
| `allowed_origins` | array | `["*"]` | 允许的 CORS 源 |
| `users` | array | `[]` | 用户列表 |

### 用户配置

每个用户包含以下字段：

| 字段 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `api_key` | string | - | 用户 API 密钥 |
| `role` | string | `"user"` | 用户角色 |

### 示例

```json
{
  "mcp": {
    "enabled": true,
    "host": "127.0.0.1",
    "port": 8080,
    "auth_enabled": true,
    "api_key": "your-api-key-here",
    "allowed_origins": ["*"],
    "users": [
      {
        "api_key": "user-key-1",
        "role": "user"
      }
    ]
  }
}
```

## 插件配置

### plugins

Zenith 支持通过外部插件来扩展格式化能力。插件配置文件位于 `plugins/` 目录下，支持以下格式：

- **JSON 格式**：单个插件配置
- **TOML 格式**：单个插件配置或插件列表配置

#### 单个插件配置（JSON）

```json
{
  "name": "prettier-js",
  "command": "prettier",
  "args": ["--stdin-filepath", "{filepath}", "--parser", "babel"],
  "extensions": ["js", "jsx", "ts", "tsx"],
  "enabled": true
}
```

#### 单个插件配置（TOML）

```toml
[plugin]
name = "prettier-js"
command = "prettier"
args = ["--stdin-filepath", "{filepath}", "--parser", "babel"]
extensions = ["js", "jsx", "ts", "tsx"]
enabled = true
```

#### 插件列表配置（TOML）

使用 `[[plugins]]` 数组语法可以在单个 TOML 文件中定义多个插件：

```toml
[[plugins]]
name = "prettier-js"
command = "prettier"
args = ["--stdin-filepath", "{filepath}", "--parser", "babel"]
extensions = ["js", "jsx", "ts", "tsx"]
enabled = true

[[plugins]]
name = "markdown-lint"
command = "markdownlint-cli2"
args = ["**/*.md", "--fix"]
extensions = ["md"]
enabled = false
```

#### 字段说明

| 字段 | 类型 | 描述 |
|------|------|------|
| `name` | string | 插件名称 |
| `command` | string | 要执行的命令（可以是命令名或路径） |
| `args` | array | 命令参数列表，支持 `{filepath}` 占位符 |
| `extensions` | array | 该插件处理的文件扩展名 |
| `enabled` | boolean | 是否启用该插件 |

#### 插件目录结构

```
plugins/
├── prettier-js.json
├── markdown-lint.json
└── plugins.toml      # 也可以合并为列表格式
```

### 默认插件位置

默认情况下，Zenith 会从以下位置加载插件：

1. `./plugins/` - 项目根目录下的 plugins 目录
2. `~/.config/zenith/plugins/` - 用户配置目录下的 plugins 目录

## 配置优先级

Zenith 按以下优先级加载配置：

1. 命令行指定的配置文件 (`--config` 参数)
2. 项目根目录的 `.zenith.json`
3. 项目根目录的 `.zenith.toml`
4. 全局默认配置

## 配置文件位置

### 项目级配置

在项目根目录创建配置文件：

- `.zenith.json` - JSON 格式配置
- `.zenith.toml` - TOML 格式配置

### 全局配置

在用户主目录创建配置文件：

- `~/.zenith/config.json` - 全局 JSON 配置
- `~/.zenith/config.toml` - 全局 TOML 配置

## 命令行选项

命令行选项会覆盖配置文件中的设置：

| 选项 | 描述 |
|------|------|
| `--config <path>` | 指定配置文件路径 |
| `--no-backup` | 禁用备份 |
| `--no-cache` | 禁用缓存 |
| `--no-recursive` | 禁用递归处理 |
| `--workers <num>` | 设置并发工作线程数 |
| `--log-level <level>` | 设置日志级别 |

## 最佳实践

### 1. 使用版本控制

将配置文件纳入版本控制，确保团队成员使用相同的配置。

### 2. 分层配置

- 在项目根目录使用基础配置
- 在子目录使用特定配置覆盖

### 3. 性能调优

根据项目规模调整并发配置：

```json
{
  "concurrency": {
    "workers": 8,
    "batch_size": 200
  }
}
```

### 4. 安全性

- 不要在配置文件中存储敏感信息
- 使用环境变量或密钥管理服务存储 API 密钥
- 限制 `allowed_origins` 为特定域名

### 5. 备份策略

根据项目需求调整备份保留天数：

```json
{
  "backup": {
    "retention_days": 30
  }
}
```

## 故障排查

### 配置未生效

1. 检查配置文件格式是否正确
2. 确认配置文件位置
3. 使用 `--log-level debug` 查看详细日志

### 格式化器未运行

1. 检查格式化器是否已安装
2. 确认格式化器在配置中已启用
3. 验证自定义配置文件路径

### 性能问题

1. 增加 `workers` 数量
2. 调整 `batch_size`
3. 检查 `max_file_size_mb` 限制
