# Zenith 🎨

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/workflow/status/user/zenith/CI)](https://github.com/user/zenith/actions)
[![Coverage](https://img.shields.io/codecov/c/github/user/zenith)](https://codecov.io/gh/user/zenith)

**高性能、多语言代码格式化工具，支持自动备份与一键恢复**

[快速开始](#快速开始) • [功能特性](#功能特性) • [安装](#安装) • [使用文档](docs/USE_GUIDE.md) • [贡献指南](#贡献)

</div>

---

## ✨ 功能特性

### 🚀 核心功能
- **多语言支持**：支持 Rust、Python、JavaScript、TypeScript、C/C++、Java、Vue、React 等 14 种语言
- **高性能处理**：1秒处理10+文件，支持智能并发
- **安全备份**：格式化前自动备份，支持一键恢复
- **灵活配置**：支持 TOML 配置文件 + 环境变量
- **双接口**：CLI 命令行 + MCP 协议

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

## 🎯 快速开始

### 安装

**方式 1：Cargo 安装（推荐）**
```bash
cargo install zenith
```

**方式 2：预编译二进制**
```bash
# Linux/macOS
curl -sSL https://github.com/user/zenith/releases/latest/download/install.sh | sh

# Windows (PowerShell)
iwr https://github.com/user/zenith/releases/latest/download/install.ps1 | iex
```

**方式 3：从源码构建**
```bash
git clone https://github.com/user/zenith.git
cd zenith
cargo build --release
sudo mv target/release/zenith /usr/local/bin/
```

### 验证安装
```bash
zenith --version
# 输出: zenith 1.0.0
```

---

## 🔥 快速示例

### 格式化单个文件
```bash
zenith format src/main.rs
```

### 格式化整个项目
```bash
zenith format ./ --recursive
```

### 检查模式（不修改文件）
```bash
zenith format src/ --check
```

### 恢复备份
```bash
zenith recover backup_20231223_142030
```

---

## 📖 详细用法

查看完整的使用指南：[USE_GUIDE.md](docs/USE_GUIDE.md)

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
```

### 配置文件示例

创建 `zenith.toml`：
```toml
[global]
backup_enabled = true
log_level = "info"
recursive = true

[zeniths.rust]
enabled = true
config_path = ".rustfmt.toml"

[zeniths.python]
enabled = true
config_path = "pyproject.toml"

[concurrency]
workers = 8
batch_size = 100

[backup]
dir = ".zenith_backup"
retention_days = 7
```

### 环境变量
```bash
export ZENITH_WORKERS=16
export ZENITH_LOG_LEVEL=debug
export ZENITH_NO_BACKUP=false

zenith format src/
```

---

## 🏗️ 架构设计
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

---

## 🚀 性能指标

| 场景 | 性能 |
|------|------|
| 单个小文件 (<10KB) | < 50ms |
| 单个中文件 (100KB) | < 200ms |
| 10 文件并发 | < 1s |
| 100 文件批处理 | < 10s |
| 1000 文件批处理 | < 100s |
| 内存占用 | < 100MB |

---

## 🛠️ 开发指南

### 前置要求
- Rust 1.75+
- 外部格式化工具（按需安装）：
  - rustfmt: `rustup component add rustfmt`
  - ruff: `pip install ruff`
  - prettier: `npm install -g prettier`
  - clang-format: 系统包管理器安装

### 本地开发
```bash
# 克隆仓库
git clone https://github.com/user/zenith.git
cd zenith

# 运行测试
cargo test

# 运行基准测试
cargo bench

# 代码覆盖率
cargo tarpaulin --out Html

# 运行工具
cargo run -- format test.rs
```

### 项目结构
```
zenith/
├── src/
│   ├── main.rs              # 入口
│   ├── cli/                 # CLI 接口
│   ├── mcp/                 # MCP 服务器
│   ├── core/                # 核心逻辑
│   ├── zeniths/          # 格式化器实现
│   ├── service/             # 业务服务
│   ├── storage/             # 存储层
│   └── utils/               # 工具函数
├── tests/                   # 测试
├── benches/                 # 基准测试
├── docs/                    # 文档
└── config/                  # 配置模板
```

---

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md)

### 如何贡献
1. Fork 本仓库
2. 创建特性分支：`git checkout -b feature/amazing-feature`
3. 提交更改：`git commit -m 'Add amazing feature'`
4. 推送到分支：`git push origin feature/amazing-feature`
5. 提交 Pull Request

### 开发规范
- 遵循 Rust 官方代码风格
- 添加单元测试（覆盖率 > 70%）
- 更新相关文档
- 通过 CI/CD 检查

---

## 📊 路线图

### ✅ v1.0.0 (当前)
- [x] 核心格式化功能
- [x] 备份恢复系统
- [x] CLI 接口
- [x] MCP 协议支持
- [x] 6 种主流语言支持

### 🔜 v1.1.0
- [ ] 增量格式化（仅格式化变更文件）
- [ ] Git Hooks 集成
- [ ] 更多语言支持（Go、Swift、Kotlin）
- [ ] Web UI 控制台

### 🎯 v2.0.0
- [ ] 分布式格式化
- [ ] 实时文件监听
- [ ] LSP 集成
- [ ] 云端配置同步

---

## ❓ 常见问题

<details>
<summary><b>Q: 支持哪些操作系统？</b></summary>

A: 支持 Linux (x86_64, ARM64)、Windows 10+ (x86_64)、macOS 11+ (x86_64, ARM64/M1)
</details>

<details>
<summary><b>Q: 如何禁用备份？</b></summary>

A: 使用 `--no-backup` 参数或设置环境变量 `ZENITH_NO_BACKUP=true`
</details>

<details>
<summary><b>Q: 格式化失败怎么办？</b></summary>

A: 工具会自动保留备份，使用 `zenith recover <backup_id>` 恢复。查看日志获取详细错误信息。
</details>

<details>
<summary><b>Q: 如何添加自定义格式化规则？</b></summary>

A: 在项目根目录创建对应的配置文件（如 `.rustfmt.toml`、`.prettierrc`），工具会自动识别。
</details>

<details>
<summary><b>Q: 支持 CI/CD 集成吗？</b></summary>

A: 支持！在 CI 中使用 `--check` 模式验证代码格式，退出码非零表示需要格式化。
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

- **Issue Tracker**: [GitHub Issues](https://github.com/user/zenith/issues)
- **Discussions**: [GitHub Discussions](https://github.com/user/zenith/discussions)
- **Email**: your.email@example.com

---

<div align="center">

**如果觉得有用，请给个 ⭐️ Star！**

Made with ❤️ by the Zenith Team

</div>