# Zenith 🎨

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/workflow/status/user/zenith/CI)](https://github.com/user/zenith/actions)
[![Coverage](https://img.shields.io/codecov/c/github/user/zenith)](https://codecov.io/gh/user/zenith)

**High-performance, multi-language code formatter with automatic backup and one-click recovery**

[Quick Start](#quick-start) • [Features](#features) • [Installation](#installation) • [User Guide](docs/USE_GUIDE.md) • [Contributing](#contributing)

</div>

---

## ✨ Features

### 🚀 Core Capabilities
- **Multi-language Support**: Supports 14+ languages including Rust, Python, JavaScript, TypeScript, C/C++, Java, Vue, React, etc.
- **High Performance**: Process 10+ files per second with intelligent concurrency.
- **Secure Backup**: Automatic backup before formatting, supporting one-click recovery.
- **Flexible Configuration**: Supports TOML configuration files + environment variables.
- **Dual Interface**: CLI command line + MCP protocol.

### 📦 Supported File Types

| Category | Language/Format | Extension | Tool |
|----------|-----------------|-----------|------|
| **Programming** | Rust | `.rs` | rustfmt |
| | Python | `.py` | ruff/black |
| | JavaScript | `.js` | prettier |
| | TypeScript | `.ts` | prettier |
| | C/C++ | `.c` `.cpp` `.h` | clang-format |
| | Java | `.java` | google-java-format |
| | Vue | `.vue` | prettier |
| | React | `.jsx` `.tsx` | prettier |
| **Configuration** | JSON | `.json` | Built-in |
| | YAML | `.yaml` `.yml` | Built-in |
| | TOML | `.toml` | taplo |
| | INI | `.ini` | Built-in |
| | Markdown | `.md` | mdformat |
| | Shell | `.sh` | shfmt |

---

## 🎯 Quick Start

### Installation

**Method 1: Cargo (Recommended)**
```bash
cargo install zenith
```

**Method 2: Pre-compiled Binaries**
```bash
# Linux/macOS
curl -sSL https://github.com/user/zenith/releases/latest/download/install.sh | sh

# Windows (PowerShell)
iwr https://github.com/user/zenith/releases/latest/download/install.ps1 | iex
```

**Method 3: Build from Source**
```bash
git clone https://github.com/user/zenith.git
cd zenith
cargo build --release
sudo mv target/release/zenith /usr/local/bin/
```

### Verify Installation
```bash
zenith --version
# Output: zenith 1.0.0
```

---

## 🔥 Quick Examples

### Format a Single File
```bash
zenith format src/main.rs
```

### Format Entire Project
```bash
zenith format ./ --recursive
```

### Check Mode (Dry Run)
```bash
zenith format src/ --check
```

### Recover from Backup
```bash
zenith recover backup_20231223_142030
```

---

## 📖 Detailed Usage

Check the full guide: [USE_GUIDE.md](docs/USE_GUIDE.md)

### Basic Commands
```bash
# Format files/directories
zenith format <PATH>...

# Recover a backup
zenith recover <BACKUP_ID>

# List all backups
zenith list-backups

# Clean expired backups
zenith clean-backups --days 7

# Start MCP server
zenith mcp
```

### Configuration Example

Create `zenith.toml`:
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

### Environment Variables
```bash
export ZENITH_WORKERS=16
export ZENITH_LOG_LEVEL=debug
export ZENITH_NO_BACKUP=false

zenith format src/
```

---

## 🏗️ Architecture Design
```
┌─────────────────────────────────────────┐
│         User Interface Layer            │
│   CLI (clap)    |    MCP Server (rmcp)  │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│            Service Layer                │
│  ZenithService | BackupService          │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│              Core Layer                 │
│  Registry | Scheduler | FileScanner     │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│        Formatter Layer (Plugin)         │
│  Rust | Python | JS | JSON | ...        │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│            Storage Layer                │
│  SnapshotStore | DiffEngine | Cache     │
└─────────────────────────────────────────┘
```

---

## 🚀 Performance Metrics

| Scenario | Performance |
|----------|-------------|
| Small File (<10KB) | < 50ms |
| Medium File (100KB) | < 200ms |
| 10 Files Concurrent | < 1s |
| 100 Files Batch | < 10s |
| 1000 Files Batch | < 100s |
| Memory Usage | < 100MB |

---

## 🛠️ Development Guide

### Prerequisites
- Rust 1.75+
- External Formatters (Install as needed):
  - rustfmt: `rustup component add rustfmt`
  - ruff: `pip install ruff`
  - prettier: `npm install -g prettier`
  - clang-format: Install via system package manager

### Local Development
```bash
# Clone the repository
git clone https://github.com/user/zenith.git
cd zenith

# Run tests
cargo test

# Run benchmarks
cargo bench

# Code coverage
cargo tarpaulin --out Html

# Run the tool
cargo run -- format test.rs
```

### Project Structure
```
zenith/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli/                 # CLI interface
│   ├── mcp/                 # MCP server
│   ├── core/                # Core logic
│   ├── zeniths/             # Formatter implementations
│   ├── service/             # Business services
│   ├── storage/             # Storage layer
│   └── utils/               # Utility functions
├── tests/                   # Tests
├── benches/                 # Benchmarks
├── docs/                    # Documentation
└── config/                  # Configuration templates
```

---

## 🤝 Contributing

Contributions are welcome! Please check [CONTRIBUTING.md](CONTRIBUTING.md)

### How to Contribute
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

### Development Standards
- Follow official Rust code style
- Add unit tests (coverage > 70%)
- Update relevant documentation
- Pass CI/CD checks

---

## 📊 Roadmap

### ✅ v1.0.0 (Current)
- [x] Core formatting functionality
- [x] Backup & recovery system
- [x] CLI interface
- [x] MCP protocol support
- [x] 6 major languages supported

### 🔜 v1.1.0
- [ ] Incremental formatting (only format changed files)
- [ ] Git Hooks integration
- [ ] More language support (Go, Swift, Kotlin)
- [ ] Web UI Console

### 🎯 v2.0.0
- [ ] Distributed formatting
- [ ] Real-time file watching
- [ ] LSP integration
- [ ] Cloud configuration sync

---

## ❓ FAQ

<details>
<summary><b>Q: Which operating systems are supported?</b></summary>

A: Supports Linux (x86_64, ARM64), Windows 10+ (x86_64), and macOS 11+ (x86_64, ARM64/M1).
</details>

<details>
<summary><b>Q: How do I disable backups?</b></summary>

A: Use the `--no-backup` flag or set the environment variable `ZENITH_NO_BACKUP=true`.
</details>

<details>
<summary><b>Q: What if formatting fails?</b></summary>

A: The tool automatically keeps backups. Use `zenith recover <backup_id>` to restore. Check the logs for detailed error information.
</details>

<details>
<summary><b>Q: How do I add custom formatting rules?</b></summary>

A: Create the corresponding configuration file (e.g., `.rustfmt.toml`, `.prettierrc`) in the project root; the tool will identify it automatically.
</details>

<details>
<summary><b>Q: Is CI/CD integration supported?</b></summary>

A: Yes! Use `--check` mode in CI to verify code format. A non-zero exit code indicates formatting is required.
</details>

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgements

Thanks to these open-source projects:
- [rustfmt](https://github.com/rust-lang/rustfmt) - Rust formatting
- [prettier](https://github.com/prettier/prettier) - JS/TS formatting
- [clap](https://github.com/clap-rs/clap) - CLI framework
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime

---

## 📞 Contact

- **Issue Tracker**: [GitHub Issues](https://github.com/user/zenith/issues)
- **Discussions**: [GitHub Discussions](https://github.com/user/zenith/discussions)
- **Email**: your.email@example.com

---

<div align="center">

**If you find this useful, please give it a ⭐️ Star!**

Made with ❤️ by the Zenith Team

</div>
