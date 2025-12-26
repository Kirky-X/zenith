<div align="center">

# ❓ Frequently Asked Questions (FAQ)

### Quick Answers to Common Questions

[🏠 Home](../README.md) • [📖 User Guide](USER_GUIDE.md) • [🐛 Report Issue](https://github.com/Kirky-X/zenith/issues)

---

</div>

## 📋 Table of Contents

- [General Questions](#general-questions)
- [Installation & Setup](#installation--setup)
- [Usage & Features](#usage--features)
- [Performance](#performance)
- [Security](#security)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [Licensing](#licensing)

---

## General Questions

<div align="center">

### 🤔 About Zenith

</div>

<details>
<summary><b>❓ What is Zenith?</b></summary>

<br>

**Zenith** is a high-performance, multi-language code formatter with automatic backup and one-click recovery. It provides:

- ✅ **14+ Language Support** - Rust, Python, Markdown, JSON, TOML, Shell, C, Java, INI, and more
- ✅ **Intelligent Caching** - Skips already-formatted files for faster processing
- ✅ **Automatic Backups** - Preserves original code before formatting
- ✅ **MCP Server** - Model Context Protocol server for AI integration
- ✅ **Plugin System** - Extensible architecture for custom formatters

It's designed for developers who need a reliable, fast, and safe code formatting solution.

**Learn more:** [User Guide](USER_GUIDE.md)

</details>

<details>
<summary><b>❓ Why should I use Zenith instead of alternatives?</b></summary>

<br>

<table>
<tr>
<th>Feature</th>
<th>Zenith</th>
<th>Prettier</th>
<th>rustfmt</th>
</tr>
<tr>
<td>Multi-language Support</td>
<td>✅ 14+ languages</td>
<td>✅ 10+ languages</td>
<td>❌ Rust only</td>
</tr>
<tr>
<td>Performance</td>
<td>⚡⚡⚡ Async + Cache</td>
<td>⚡⚡</td>
<td>⚡⚡</td>
</tr>
<tr>
<td>Automatic Backup</td>
<td>✅ Built-in</td>
<td>❌</td>
<td>❌</td>
</tr>
<tr>
<td>Smart Caching</td>
<td>✅ Intelligent</td>
<td>❌</td>
<td>❌</td>
</tr>
<tr>
<td>MCP Integration</td>
<td>✅ Native support</td>
<td>❌</td>
<td>❌</td>
</tr>
<tr>
<td>Rust Implementation</td>
<td>✅ Native</td>
<td>❌ Node.js</td>
<td>✅ Native</td>
</tr>
</table>

**Key Advantages:**
- 🚀 **Fast** - Built in Rust with async/await and intelligent caching
- 🔒 **Safe** - Automatic backups before formatting, one-click recovery
- 🧩 **Extensible** - Plugin system for custom formatters
- 🤖 **AI-Ready** - MCP server for AI assistant integration

</details>

<details>
<summary><b>❓ Is Zenith production-ready?</b></summary>

<br>

**Current Status:** ✅ **Yes, production-ready!**

<table>
<tr>
<td width="50%">

**What's Ready:**
- ✅ Core formatting functionality stable
- ✅ 14+ language formatters available
- ✅ Backup and recovery system
- ✅ MCP server for AI integration
- ✅ Comprehensive configuration options

</td>
<td width="50%">

**Maturity Indicators:**
- 📊 Well-tested Rust codebase
- 🔄 Active development
- 📝 MIT Licensed
- 👥 Open source on GitHub
- 🐛 Issue tracking and support

</td>
</tr>
</table>

> **Note:** Always review the [CHANGELOG](../CHANGELOG.md) before upgrading versions.

</details>

<details>
<summary><b>❓ What platforms are supported?</b></summary>

<br>

<table>
<tr>
<th>Platform</th>
<th>Architecture</th>
<th>Status</th>
<th>Notes</th>
</tr>
<tr>
<td rowspan="2"><b>Linux</b></td>
<td>x86_64</td>
<td>✅ Fully Supported</td>
<td>Primary platform</td>
</tr>
<tr>
<td>ARM64</td>
<td>✅ Fully Supported</td>
<td>Tested on ARM servers</td>
</tr>
<tr>
<td rowspan="2"><b>macOS</b></td>
<td>x86_64</td>
<td>✅ Fully Supported</td>
<td>Intel Macs</td>
</tr>
<tr>
<td>ARM64</td>
<td>✅ Fully Supported</td>
<td>Apple Silicon (M1/M2)</td>
</tr>
<tr>
<td><b>Windows</b></td>
<td>x86_64</td>
<td>✅ Fully Supported</td>
<td>Windows 10+</td>
</tr>
</table>

</details>

<details>
<summary><b>❓ What programming languages are supported?</b></summary>

<br>

<table>
<tr>
<td width="33%" align="center">

**🦀 Rust**

✅ **Native Support**

Full support with rustfmt

</td>
<td width="33%" align="center">

**🐍 Python**

✅ **Native Support**

Full support with ruff

</td>
<td width="33%" align="center">

**📝 Markdown**

✅ **Native Support**

Built-in formatter

</td>
</tr>
<tr>
<td width="33%" align="center">

**📋 JSON/TOML**

✅ **Native Support**

Built-in formatters

</td>
<td width="33%" align="center">

**🔧 Shell**

✅ **Native Support**

sh/bash formatting

</td>
<td width="33%" align="center">

**©️ C/C++**

✅ **Native Support**

With clang-format

</td>
</tr>
<tr>
<td width="33%" align="center">

**☕ Java**

✅ **Native Support**

Java formatting

</td>
<td width="33%" align="center">

**⚙️ INI**

✅ **Native Support**

INI file formatting

</td>
<td width="33%" align="center">

**🎨 Prettier**

🚧 **Plugin**

Via external integration

</td>
</tr>
</table>

**Documentation:**
- [User Guide](USER_GUIDE.md#core-concepts)
- [Examples](../examples/)

</details>

---

## Installation & Setup

<div align="center">

### 🚀 Getting Started

</div>

<details>
<summary><b>❓ How do I install Zenith?</b></summary>

<br>

**Using Cargo Install (Recommended):**

```bash
cargo install --git https://github.com/Kirky-X/zenith.git
```

**From Source:**

```bash
git clone https://github.com/Kirky-X/zenith.git
cd zenith
cargo build --release
```

**Quick Install Script (Linux/macOS):**

```bash
curl -sSL https://raw.githubusercontent.com/Kirky-X/zenith/main/install.sh | sh
```

**Windows (PowerShell):**

```powershell
iwr -useb https://raw.githubusercontent.com/Kirky-X/zenith/main/install.ps1 | iex
```

**Verification:**

```bash
zenith --version
# Output: zenith 0.1.0

zenith doctor
# Checks environment and dependencies
```

**See also:** [Installation Guide](USER_GUIDE.md#installation)

</details>

<details>
<summary><b>❓ What are the system requirements?</b></summary>

<br>

**Minimum Requirements:**

<table>
<tr>
<th>Component</th>
<th>Requirement</th>
<th>Recommended</th>
</tr>
<tr>
<td>Rust Version</td>
<td>1.75+</td>
<td>Latest stable</td>
</tr>
<tr>
<td>Memory</td>
<td>512 MB</td>
<td>2 GB+</td>
</tr>
<tr>
<td>Disk Space</td>
<td>50 MB</td>
<td>100 MB</td>
</tr>
<tr>
<td>CPU</td>
<td>1 core</td>
<td>4+ cores</td>
</tr>
</table>

**Optional:**
- 🔧 Formatter tools (rustfmt, ruff, clang-format) for specific languages
- 🐳 Docker (for containerized deployment)

</details>

<details>
<summary><b>❓ How do I configure Zenith?</b></summary>

<br>

**Initialize Configuration:**

```bash
zenith init
```

**Create `zenith.toml`:**

```toml
[global]
backup_enabled = true
log_level = "info"
recursive = true
cache_enabled = true
config_dir = ".zenith"

[zeniths.rust]
enabled = true
config_path = ".rustfmt.toml"
use_default = true

[zeniths.python]
enabled = true
config_path = "pyproject.toml"
use_default = true

[concurrency]
workers = 8
batch_size = 100

[backup]
dir = ".zenith_backup"
retention_days = 7

[mcp]
enabled = false
host = "127.0.0.1"
port = 9000
auth_enabled = false
```

**Environment Variables:**

```bash
export ZENITH_WORKERS=16
export ZENITH_LOG_LEVEL=debug
export ZENITH_NO_BACKUP=false
export ZENITH_RECURSIVE=true
```

**See also:** [Configuration Guide](USER_GUIDE.md#configuration)

</details>

<details>
<summary><b>❓ Can I use Zenith in CI/CD?</b></summary>

<br>

**Yes!** Here's how to integrate with GitHub Actions:

```yaml
name: Format Check

on: [push, pull_request]

jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Zenith
        run: curl -sSL https://raw.githubusercontent.com/Kirky-X/zenith/main/install.sh | sh
      - name: Check formatting
        run: zenith format ./ --recursive --check
```

**GitLab CI:**

```yaml
format:
  stage: test
  script:
    - curl -sSL https://raw.githubusercontent.com/Kirky-X/zenith/main/install.sh | sh
    - zenith format ./ --recursive --check
```

**Best Practices:**
- Use `--check` mode in CI to validate without modifying files
- Enable backups in production for safety
- Use caching for faster CI runs

</details>

---

## Usage & Features

<div align="center">

### 💡 Working with Zenith

</div>

<details>
<summary><b>❓ How do I format code with Zenith?</b></summary>

<br>

**Basic Usage:**

```bash
# Format a single file
zenith format src/main.rs

# Format a directory
zenith format src/

# Format recursively
zenith format ./ --recursive

# Check mode (dry-run, no changes)
zenith format src/ --check

# Disable backup for faster operation
zenith format ./ --recursive --no-backup

# Specify number of workers
zenith format ./ --workers 16
```

**Initialize Project:**

```bash
# Simple initialization
zenith init

# Overwrite existing configuration
zenith init --force

# Check environment
zenith doctor
```

**Backup and Recovery:**

```bash
# List available backups
zenith list-backups

# Restore from backup
zenith recover backup_20231223_142030

# Clean old backups
zenith clean-backups --days 7
```

**See also:** [User Guide](USER_GUIDE.md#basic-operations)

</details>

<details>
<summary><b>❓ What is the MCP server and how do I use it?</b></summary>

<br>

**MCP (Model Context Protocol)** enables AI assistants to interact with Zenith.

**Start MCP Server:**

```bash
zenith mcp
zenith mcp --addr 127.0.0.1:9000
```

**API Request:**

```bash
curl -X POST http://127.0.0.1:9000 \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "format",
    "params": {
      "paths": ["src/main.rs"],
      "recursive": false,
      "backup": true
    }
  }'
```

**Configuration:**

```toml
[mcp]
enabled = true
host = "127.0.0.1"
port = 9000
auth_enabled = false
api_key = null
allowed_origins = ["*"]
```

**Use Cases:**
- AI-assisted code formatting
- Automated code review workflows
- IDE integration via MCP protocol

</details>

<details>
<summary><b>❓ How does the backup system work?</b></summary>

<br>

**Automatic Backups:**

Zenith automatically creates backups before formatting files.

```toml
[backup]
dir = ".zenith_backup"
retention_days = 7
```

**Backup Commands:**

```bash
# List all backups
zenith list-backups

# Restore from specific backup
zenith recover backup_20231223_142030

# Clean backups older than 7 days
zenith clean-backups --days 7

# Auto rollback to latest backup
zenith auto-rollback
```

**Backup Structure:**

```
.zenith_backup/
├── backup_20231223_142030/
│   ├── src_main.rs
│   └── src_lib.rs
├── backup_20231223_143000/
│   └── src_main.rs
```

</details>

<details>
<summary><b>❓ How does the caching system improve performance?</b></summary>

<br>

**How It Works:**

Zenith tracks file hashes and skips already-formatted files.

```toml
[global]
cache_enabled = true
```

**Performance Comparison:**

<table>
<tr>
<td width="50%">

**Without Cache**
```bash
zenith format ./ --recursive
# Time: 10s (all files)
```

</td>
<td width="50%">

**With Cache**
```bash
zenith format ./ --recursive
# Time: 1s (only changed files)
```

</td>
</tr>
</table>

**Cache Management:**

- Cache is automatic - no manual intervention needed
- Cache invalidates when file content changes
- Disable with `ZENITH_CACHE_ENABLED=false` if needed

</details>

<details>
<summary><b>❓ How do I add custom formatters via plugins?</b></summary>

<br>

**Plugin System:**

Zenith supports custom formatters through its plugin architecture.

**Create Custom Zenith:**

```rust
use zenith::core::traits::Zenith;

pub struct CustomZenith;

impl Zenith for CustomZenith {
    fn name(&self) -> &'static str {
        "custom"
    }
    
    fn extensions(&self) -> &'static [&'static str] {
        &["custom"]
    }
    
    fn format(
        &self,
        content: &str,
        config: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Your formatting logic
        Ok(formatted_content)
    }
}
```

**Security Configuration:**

```toml
[security]
allowed_plugin_commands = ["custom-formatter"]
allow_absolute_paths = false
allow_relative_paths = false
```

</details>

---

## Performance

<div align="center">

### ⚡ Optimization

</div>

<details>
<summary><b>❓ How can I optimize Zenith for large projects?</b></summary>

<br>

**Increase Workers:**

```bash
# Match CPU cores
zenith format ./ --workers $(nproc)

# For very large projects
zenith format ./ --workers 32
```

**Configuration:**

```toml
[concurrency]
workers = 16
batch_size = 200
```

**Disable Backup for CI:**

```bash
zenith format ./ --recursive --no-backup
```

**Selective Formatting:**

```bash
# Only specific file types
zenith format $(find ./ -name "*.rs" -o -name "*.py")

# Exclude directories
zenith format ./ --recursive --exclude target --exclude node_modules
```

**Performance Profiles:**

<table>
<tr>
<th>Profile</th>
<th>Use Case</th>
<th>Throughput</th>
<th>Memory</th>
</tr>
<tr>
<td><b>Default</b></td>
<td>General purpose</td>
<td>High</td>
<td>Medium</td>
</tr>
<tr>
<td><b>High Throughput</b></td>
<td>Large projects</td>
<td>⚡ Very High</td>
<td>High</td>
</tr>
<tr>
<td><b>Low Memory</b></td>
<td>Resource-constrained</td>
<td>Low</td>
<td>⚡ Very Low</td>
</tr>
</table>

</details>

<details>
<summary><b>❓ What are the best practices for performance?</b></summary>

<br>

**✅ DO's:**

- Use `--workers` to match your CPU cores
- Enable caching for repeated formatting
- Use `--check` mode in CI pipelines
- Exclude build directories (`target`, `node_modules`)
- Keep backups enabled for safety

**❌ DON'Ts:**

- Don't use debug logging in production
- Don't disable caching for small projects
- Don't format entire filesystem without recursion control

**Tips:**

```bash
# Check performance with verbose logging
zenith format ./ --verbose

# Monitor with debug level
ZENITH_LOG_LEVEL=debug zenith format ./
```

</details>

---

## Security

<div align="center">

### 🔒 Security Best Practices

</div>

<details>
<summary><b>❓ Is Zenith safe to use?</b></summary>

<br>

**Yes!** Zenith is designed with safety in mind:

- ✅ **Automatic Backups** - Original code preserved before formatting
- ✅ **One-Click Recovery** - Easily restore from backups
- ✅ **Sandboxed Plugins** - Plugin security configuration
- ✅ **Safe Defaults** - Backup enabled by default

**Security Features:**

<table>
<tr>
<td width="50%">

**Code Safety:**
- Automatic backups before formatting
- Dry-run mode (`--check`) available
- Versioned backup storage
- Configurable retention policy

</td>
<td width="50%">

**Plugin Security:**
- Command whitelisting
- Path access control
- Secure defaults
- Audit logging

</td>
</tr>
</table>

</details>

<details>
<summary><b>❓ How do I secure the MCP server?</b></summary>

<br>

**Enable Authentication:**

```toml
[mcp]
enabled = true
auth_enabled = true
api_key = "${ZENITH_API_KEY}"
allowed_origins = ["https://your-domain.com"]
```

**Use Environment Variables:**

```bash
export ZENITH_API_KEY="your-secure-api-key"
```

**Don't commit API keys:**

```toml
# ❌ Bad - exposes API key
api_key = "secret-key"

# ✅ Good - uses environment variable
api_key = "${ZENITH_API_KEY}"
```

**Security Recommendations:**
- Always enable authentication in production
- Use strong API keys
- Restrict allowed origins
- Monitor MCP server access

</details>

<details>
<summary><b>❓ How do I handle sensitive files?</b></summary>

<br>

**Exclude Sensitive Files:**

```bash
# Exclude specific directories
zenith format ./ --exclude .env --exclude secrets --exclude credentials
```

**Configuration:**

```toml
[limits]
max_file_size_mb = 10
```

**Best Practices:**
- Don't format files containing secrets
- Use `.gitignore` to exclude sensitive directories
- Review changes before committing
- Use backup system for safety

</details>

---

## Troubleshooting

<div align="center">

### 🔧 Common Issues

</div>

<details>
<summary><b>❓ Formatter not found</b></summary>

<br>

**Problem:** Missing formatter tool for a specific language.

**Solution:**

```bash
# Check environment
zenith doctor

# Install missing formatters:
# Rust
rustup component add rustfmt

# Python
pip install ruff

# C/C++
apt install clang-format

# Prettier
npm install -g prettier
```

**Configure external formatters:**

```toml
[zeniths.python]
enabled = true
config_path = "pyproject.toml"
```

</details>

<details>
<summary><b>❓ Permission denied</b></summary>

<br>

**Problem:** File access issues.

**Solution:**

```bash
# Check file permissions
ls -la path/to/file

# Fix permissions
chmod +w path/to/file

# Or use sudo (caution)
sudo zenith format /etc/config/file.conf
```

**Note:** Be careful with sudo - only use when necessary.

</details>

<details>
<summary><b>❓ Backup directory full</b></summary>

<br>

**Problem:** Disk space issues from backups.

**Solution:**

```bash
# Clean old backups
zenith clean-backups --days 7

# Or manually remove
rm -rf .zenith_backup/*

# Configure retention
[backup]
retention_days = 3
```

</details>

<details>
<summary><b>❓ Slow performance</b></summary>

<br>

**Problem:** Formatting takes too long.

**Diagnosis & Solution:**

```bash
# Increase workers
zenith format ./ --recursive --workers 32

# Disable cache temporarily
ZENITH_CACHE_ENABLED=false zenith format ./ --recursive

# Exclude build directories
zenith format ./ --recursive --exclude target --exclude node_modules

# Check with verbose mode
zenith format ./ --verbose
```

**Performance Tips:**
- Match workers to CPU cores
- Enable caching for repeated runs
- Exclude build and dependency directories
- Use selective formatting for large projects

</details>

<details>
<summary><b>❓ Configuration errors</b></summary>

<br>

**Problem:** Invalid configuration file.

**Solution:**

```bash
# Validate configuration
zenith doctor

# Check config syntax
cat zenith.toml

# Reset to defaults
zenith init --force
```

**Common Issues:**
- Invalid TOML syntax
- Missing required fields
- Invalid enum values
- File path issues

</details>

<div align="center">

**💬 Still need help?** [Open an issue](https://github.com/Kirky-X/zenith/issues) or [join our Discussions](https://github.com/Kirky-X/zenith/discussions)

</div>

---

## Contributing

<div align="center">

### 🤝 Help Improve Zenith

</div>

<details>
<summary><b>❓ How can I contribute?</b></summary>

<br>

**Ways to Contribute:**

- 🐛 **Bug Reports** - Report issues on GitHub
- 💡 **Feature Requests** - Suggest new features
- 📝 **Documentation** - Improve docs and guides
- 🔧 **Code Contributions** - Submit pull requests
- 🧪 **Testing** - Help test new features

**Getting Started:**

```bash
# Fork the repository
git clone https://github.com/YOUR-USERNAME/zenith.git
cd zenith

# Create a feature branch
git checkout -b feature/amazing-feature

# Make changes and test
cargo test

# Submit pull request
git push origin feature/amazing-feature
```

**See also:** [CONTRIBUTING.md](../CONTRIBUTING.md)

</details>

<details>
<summary><b>❓ What coding standards does Zenith follow?</b></summary>

<br>

**Rust Standards:**

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Enable `clippy::all` and `clippy::pedantic`
- Write documentation for public APIs
- Add tests for new functionality

**Code Style:**

```bash
# Format code
cargo fmt

# Run lints
cargo clippy

# Run tests
cargo test
```

</details>

---

## Licensing

<div align="center">

### 📜 License Information

</div>

<details>
<summary><b>❓ What license is Zenith under?</b></summary>

<br>

**MIT License**

Zenith is licensed under the MIT License. This is a permissive license that allows:

- ✅ Commercial use
- ✅ Modification
- ✅ Distribution
- ✅ Private use
- ✅ Sublicensing

**Required:**
- Include copyright notice
- Include license notice

**No Liability:**
- THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY

**Full License:** See [LICENSE](../LICENSE) file for details.

</details>

<details>
<summary><b>❓ Can I use Zenith in my commercial project?</b></summary>

<br>

**Yes!** The MIT License allows commercial use:

- ✅ Use Zenith in commercial products
- ✅ Modify Zenith for your needs
- ✅ Distribute modified versions
- ✅ No attribution required (but appreciated)

**Example Commercial Use:**
- Internal company tooling
- SaaS products
- Open-source libraries
- Closed-source applications

</details>

---

<div align="center">

**[📖 User Guide](USER_GUIDE.md)** • **[🏠 Home](../README.md)** • **[🐛 Report Issue](https://github.com/Kirky-X/zenith/issues)**

Made with ❤️ by Kirky-X

[⬆ Back to Top](#-frequently-asked-questions-faq)

</div>
