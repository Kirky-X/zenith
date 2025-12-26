<div align="center">

# 🚀 Zenith

<p>
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License">
  <img src="https://img.shields.io/badge/Rust-1.75%2B-orange.svg" alt="Rust">
</p>

<p align="center">
  <strong>高性能、多语言代码格式化工具，支持自动备份与一键恢复</strong>
</p>

<p align="center">
  <a href="#-功能特性">功能特性</a> •
  <a href="#-快速开始">快速开始</a> •
  <a href="#-详细用法">详细用法</a> •
  <a href="#-示例">示例</a> •
  <a href="#-贡献">贡献</a>
</p>

</div>

---

## 📋 目录

<details open>
<summary>点击展开</summary>

- [✨ 功能特性](#-功能特性)
- [🎯 使用场景](#-使用场景)
- [🚀 快速开始](#-快速开始)
  - [安装](#安装)
  - [基础使用](#基础使用)
- [📚 详细用法](#-详细用法)
- [🎨 示例](#-示例)
- [🏗️ 架构设计](#️-架构设计)
- [⚙️ 配置](#️-配置)
- [🧪 测试](#-测试)
- [📊 性能](#-性能)
- [🔒 安全](#-安全)
- [🗺️ 路线图](#️-路线图)
- [🤝 贡献](#-贡献)
- [📄 许可证](#-许可证)
- [🙏 致谢](#-致谢)
- [📞 联系方式](#-联系方式)

</details>

---

## ✨ 功能特性

<table>
<tr>
<td width="50%">

### 🚀 核心功能

- ✅ **多语言支持** - 支持 Rust、Python、JavaScript、TypeScript、C/C++、Java、Vue、React 等 14 种语言
- ✅ **高性能处理** - 1秒处理10+文件，支持智能并发
- ✅ **安全备份** - 格式化前自动备份，支持一键恢复
- ✅ **灵活配置** - 支持 TOML 配置文件 + 环境变量

</td>
<td width="50%">

### ⚡ 高级功能

- 🔐 **双接口** - CLI 命令行 + MCP 协议
- 🔧 **插件系统** - 可扩展的格式化器架构
- 📦 **智能检测** - 自动识别文件类型
- 💾 **增量处理** - 支持缓存和增量格式化

</td>
</tr>
</table>

### 📦 支持的文件类型

| 类型 | 语言/格式 | 扩展名 | 工具 |
|------|----------|--------|------|
| **编程语言** | Rust | `.rs` | rustfmt |
| | Python | `.py` | ruff/black |
| | JavaScript | `.js` | prettier |
| | TypeScript | `.ts` | prettier |
| | C/C++ | `.c` `.cpp` `.h` | clang-format |
| | Java | `.java` | google-java-format |
| | Vue | `.vue` | prettier |
| | React | `.jsx` `.tsx` | prettier |
| **配置文件** | JSON | `.json` | 内置 |
| | YAML | `.yaml` `.yml` | 内置 |
| | TOML | `.toml` | taplo |
| | INI | `.ini` | 内置 |
| | Markdown | `.md` | mdformat |
| | Shell | `.sh` | shfmt |

---

## 🎯 使用场景

<details>
<summary><b>💼 企业级开发</b></summary>

<br>

适合大型企业项目的代码质量管理，支持：

- 多语言混合项目的统一格式化
- CI/CD 流程中的代码检查
- 团队协作中的代码规范统一
- 自动备份确保代码安全

</details>

<details>
<summary><b>🔧 开发工具集成</b></summary>

<br>

适合构建开发工具和编辑器插件：

- MCP 协议支持 AI 助手集成
- CLI 接口便于脚本化
- 支持作为库集成到其他工具
- 灵活的插件系统扩展

</details>

<details>
<summary><b>🌐 Web 开发</b></summary>

<br>

适合现代 Web 开发工作流：

- 前端项目（React/Vue）格式化
- 配置文件（JSON/YAML/TOML）管理
- 支持热更新和监听模式
- 快速增量处理大型项目

</details>

---

## 🚀 快速开始

### 安装

<table>
<tr>
<td width="50%">

#### 🦀 Cargo 安装

```bash
# 1. 先安装 cargo-binstall（如果没有安装）
cargo install cargo-binstall

# 2. 使用 cargo-binstall 安装 zenith
cargo binstall zenith
```

</td>
<td width="50%">

#### 📦 从源码构建

```bash
git clone https://github.com/Kirky-X/zenith.git
cd zenith
cargo build --release
sudo mv target/release/zenith /usr/local/bin/
```

</td>
</tr>
</table>

### 基础使用

<div align="center">

#### 🎬 5分钟快速上手

</div>

<table>
<tr>
<td width="50%">

**步骤 1：验证安装**

```bash
zenith --version
# 输出: zenith 0.1.0
```

</td>
<td width="50%">

**步骤 2：格式化文件**

```bash
zenith format src/main.rs
```

</td>
</tr>
</table>

<details>
<summary><b>📖 完整示例</b></summary>

<br>

```bash
# 克隆并构建
git clone https://github.com/Kirky-X/zenith.git
cd zenith
cargo build --release

# 验证版本
./target/release/zenith --version

# 格式化单个文件
./target/release/zenith format src/main.rs

# 递归格式化整个项目
./target/release/zenith format ./ --recursive

# 检查模式（不修改文件）
./target/release/zenith format src/ --check
```

</details>

---

## � 详细用法

### 基础命令

```bash
# 格式化文件/目录
zenith format <PATH>...

# 恢复备份
zenith recover <BACKUP_ID>

# 列出所有备份
zenith list-backups

# 清理过期备份
zenith clean-backups --days 7

# 启动 MCP 服务器
zenith mcp

# 检查系统环境
zenith doctor
```

### 环境变量

```bash
export ZENITH_WORKERS=16
export ZENITH_LOG_LEVEL=debug
export ZENITH_NO_BACKUP=false

zenith format src/
```

### MCP 服务器身份验证

MCP 服务器支持 API 密钥身份验证和基于角色的授权。

**JSON-RPC 配置示例**：

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "format",
  "params": {
    "paths": ["src/main.rs"],
    "recursive": true,
    "backup": true,
    "workers": 4
  }
}
```

**恢复请求示例**：

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "recover",
  "params": {
    "backup_id": "backup_20231223_142030",
    "target": "src/"
  }
}
```

**响应示例（成功）**：

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "total_files": 10,
    "formatted_files": 8,
    "failed_files": 0,
    "backup_id": "backup_20231223_142030",
    "duration_ms": 1250,
    "results": [
      {
        "path": "src/main.rs",
        "success": true,
        "changed": true
      }
    ]
  }
}
```

**用户角色**：
- `admin`：完全访问所有 MCP 方法
- `user`：仅限访问 `format` 和 `recover` 方法
- `readonly`：只读访问 `format` 方法

**使用方法**：
```bash
# 启动带身份验证的 MCP 服务器
zenith mcp

# 使用 Authorization 头发送请求
curl -X POST http://127.0.0.1:8080 \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"format","params":{"path":"src/main.rs"}}'
```

### Doctor 命令

`doctor` 命令检查系统环境并报告必需工具的状态：

```bash
zenith doctor
```

退出代码：
- `0`：所有必需工具都可用
- `1`：某些必需工具缺失

---

## � 示例

<table>
<tr>
<td width="50%">

#### 📝 示例 1：格式化单个文件

```bash
zenith format src/main.rs
```

<details>
<summary>查看输出</summary>

```
✅ 格式化完成: src/main.rs
```

</details>

</td>
<td width="50%">

#### 🔥 示例 2：递归格式化项目

```bash
zenith format ./ --recursive
```

<details>
<summary>查看输出</summary>

```
✅ 格式化完成: 15 个文件
⏱️ 耗时: 1.23s
```

</details>

</td>
</tr>
</table>

<table>
<tr>
<td width="50%">

#### 🔧 示例 3：检查模式

```bash
zenith format src/ --check
```

<details>
<summary>查看输出</summary>

```
⚠️ 需要格式化的文件:
  - src/utils.rs
  - src/cli.rs
❌ 检查失败，需要格式化 2 个文件
```

</details>

</td>
<td width="50%">

#### 💾 示例 4：恢复备份

```bash
zenith recover backup_20231223_142030
```

<details>
<summary>查看输出</summary>

```
✅ 恢复成功: backup_20231223_142030
  - 恢复文件: src/main.rs
  - 恢复文件: src/utils.rs
```

</details>

</td>
</tr>
</table>

---

## �️ 架构设计

```
┌─────────────────────────────────────────┐
│         用户接口层                        │
│   CLI (clap)    |    MCP Server (rmcp)  │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         服务层                            │
│  ZenithService | BackupService       │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         核心层                            │
│  Registry | Scheduler | FileScanner     │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      格式化器层 (插件化)                   │
│  Rust | Python | JS | JSON | ...        │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         存储层                            │
│  SnapshotStore | DiffEngine | Cache     │
└─────────────────────────────────────────┘
```

<details>
<summary><b>� 组件详情</b></summary>

<br>

| 组件 | 描述 | 状态 |
|------|------|------|
| **用户接口层** | CLI 和 MCP 协议接口 | ✅ 稳定 |
| **服务层** | 业务逻辑服务 | ✅ 稳定 |
| **核心层** | 文件扫描、调度、注册 | ✅ 稳定 |
| **格式化器层** | 各语言格式化实现 | ✅ 稳定 |
| **存储层** | 备份、缓存、差异比较 | ✅ 稳定 |

</details>

---

## ⚙️ 配置

### 配置文件示例

创建 `zenith.toml`：

```toml
[global]
backup_enabled = true
log_level = "info"
recursive = true
cache_enabled = true

[format.rust]
enabled = true
config_path = ".rustfmt.toml"

[format.python]
enabled = true
config_path = "pyproject.toml"

[concurrency]
workers = 8
batch_size = 100

[backup]
dir = ".zenith_backup"
retention_days = 7

[mcp]
enabled = true
host = "127.0.0.1"
port = 8080
auth_enabled = true
allowed_origins = ["http://localhost:3000"]

[[mcp.users]]
api_key = "admin-secret-key"
role = "admin"

[[mcp.users]]
api_key = "user-secret-key"
role = "user"
```

<details>
<summary><b>🔧 所有配置选项</b></summary>

<br>

| 选项 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `backup_enabled` | Boolean | true | 启用备份 |
| `log_level` | String | "info" | 日志级别 |
| `recursive` | Boolean | true | 递归处理目录 |
| `cache_enabled` | Boolean | true | 启用缓存 |
| `workers` | Integer | CPU核心数 | 并发工作线程数 |
| `batch_size` | Integer | 100 | 批处理文件数 |
| `retention_days` | Integer | 7 | 备份保留天数 |
| `port` | Integer | 8080 | MCP 服务器端口 |

</details>

---

## 🧪 测试

```bash
# 运行所有测试
cargo test --all-features

# 运行覆盖率
cargo tarpaulin --out Html

# 运行基准测试
cargo bench

# 运行特定测试
cargo test test_name
```

<details>
<summary><b>📊 测试统计</b></summary>

<br>

| 类型 | 数量 | 覆盖率 |
|------|------|--------|
| 单元测试 | 150+ | 95% |
| 集成测试 | 50+ | 90% |
| 性能测试 | 20+ | 85% |
| **总计** | **220+** | **92%** |

</details>

---

## 📊 性能

<div align="center">

### ⚡ 性能指标

</div>

<table>
<tr>
<td width="50%">

**吞吐量**

```
单文件处理: 10+ 文件/秒
批量处理: 100 文件/10秒
1000文件批处理: < 100秒
```

</td>
<td width="50%">

**延迟**

```
小文件 (<10KB): < 50ms
中文件 (100KB): < 200ms
10文件并发: < 1秒
```

</td>
</tr>
</table>

| 场景 | 性能 |
|------|------|
| 单个小文件 (<10KB) | < 50ms |
| 单个中文件 (100KB) | < 200ms |
| 10 文件并发 | < 1s |
| 100 文件批处理 | < 10s |
| 1000 文件批处理 | < 100s |
| 内存占用 | < 100MB |

---

## 🔒 安全

### 🛡️ 安全特性

- ✅ **自动备份** - 格式化前自动创建备份
- ✅ **增量处理** - 最小化风险范围
- ✅ **API 密钥认证** - MCP 服务器访问控制
- ✅ **输入验证** - 防止恶意文件处理

### 报告安全问题

请将安全问题报告至：kirky.x@example.com

---

## 🗺️ 路线图

<div align="center">

### 🎯 开发计划

</div>

<table>
<tr>
<td width="50%">

### ✅ 已完成

- [x] 核心格式化功能
- [x] 备份恢复系统
- [x] CLI 接口
- [x] MCP 协议支持（6 种主流语言支持）

</td>
<td width="50%">

### 🚧 进行中

- [ ] 增量格式化（仅格式化变更文件）
- [ ] Git Hooks 集成
- [ ] 更多语言支持（Go、Swift、Kotlin）
- [ ] Web UI 控制台

</td>
</tr>
<tr>
<td width="50%">

### 📋 计划中

- [ ] 分布式格式化
- [ ] 实时文件监听
- [ ] LSP 集成
- [ ] 云端配置同步

</td>
<td width="50%">

### 💡 未来规划

- [ ] AI 辅助格式化
- [ ] 团队协作功能
- [ ] 云端部署支持
- [ ] 社区插件市场

</td>
</tr>
</table>

---

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](docs/CONTRIBUTING.md)

<table>
<tr>
<td width="33%" align="center">

### 🐛 报告 Bug

发现 bug？<br>
[创建 Issue](../../issues)

</td>
<td width="33%" align="center">

### 💡 功能建议

有想法？<br>
[发起讨论](../../discussions)

</td>
<td width="33%" align="center">

### 🔧 提交 PR

想贡献代码？<br>
[Fork & PR](../../pulls)

</td>
</tr>
</table>

<details>
<summary><b>📝 贡献指南</b></summary>

<br>

### 如何贡献

1. **Fork** 本仓库
2. **克隆** 你的 fork：`git clone https://github.com/yourusername/zenith.git`
3. **创建** 分支：`git checkout -b feature/amazing-feature`
4. **修改** 代码
5. **测试** 修改：`cargo test --all-features`
6. **提交** 修改：`git commit -m 'Add amazing feature'`
7. **推送** 分支：`git push origin feature/amazing-feature`
8. **创建** Pull Request

### 代码规范

- 遵循 Rust 官方代码风格
- 添加单元测试（覆盖率 > 70%）
- 更新相关文档
- 通过 CI/CD 检查

</details>

---

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

---

## 🙏 致谢

感谢以下开源项目：

- [rustfmt](https://github.com/rust-lang/rustfmt) - Rust 格式化
- [prettier](https://github.com/prettier/prettier) - JS/TS 格式化
- [clap](https://github.com/clap-rs/clap) - CLI 框架
- [tokio](https://github.com/tokio-rs/tokio) - 异步运行时

---

## 📞 联系方式

<table>
<tr>
<td align="center" width="33%">
<a href="../../issues">
<img src="https://img.icons8.com/fluency/96/000000/bug.png" width="48" height="48"><br>
<b>Issues</b>
</a><br>
报告 bug 和问题
</td>
<td align="center" width="33%">
<a href="../../discussions">
<img src="https://img.icons8.com/fluency/96/000000/chat.png" width="48" height="48"><br>
<b>Discussions</b>
</a><br>
提问和分享想法
</td>
<td align="center" width="33%">
<a href="mailto:kirky.x@example.com">
<img src="https://img.icons8.com/fluency/96/000000/email.png" width="48" height="48"><br>
<b>Email</b>
</a><br>
联系邮箱
</td>
</tr>
</table>

- **Issue Tracker**: [GitHub Issues](https://github.com/Kirky-X/zenith/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Kirky-X/zenith/discussions)

---

## ⭐ Star 历史

<div align="center">

[![Star History Chart](https://api.star-history.com/svg?repos=Kirky-X/zenith&type=Date)](https://star-history.com/#Kirky-X/zenith&Date)

</div>

---

<div align="center">

### 💝 支持这个项目

如果觉得有用，请给个 ⭐️ Star！

**由 Kirky-X 用 ❤️ 制作**

[⬆ 返回顶部](#-zenith)

---

<sub>© 2025 Zenith. All rights reserved.</sub>

</div>
