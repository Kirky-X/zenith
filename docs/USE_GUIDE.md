# Zenith User Guide

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Configuration](#configuration)
4. [Commands Reference](#commands-reference)
5. [Advanced Usage](#advanced-usage)
6. [MCP Server](#mcp-server)
7. [Troubleshooting](#troubleshooting)
8. [Best Practices](#best-practices)

---

## Installation

### System Requirements

- **Linux**: x86_64, ARM64
- **macOS**: 11+ (x86_64, ARM64/M1)
- **Windows**: 10+ (x86_64)
- **Rust**: 1.75+ (for building from source)

### Installation Methods

#### Method 1: Quick Install Script (Recommended)

**Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/user/zenith/main/install.sh | sh
```

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/user/zenith/main/install.ps1 | iex
```

#### Method 2: Cargo Install

```bash
cargo install zenith
```

#### Method 3: Pre-compiled Binaries

1. Visit [Releases](https://github.com/user/zenith/releases)
2. Download the binary for your platform
3. Extract and add to PATH:

**Linux/macOS:**
```bash
tar -xzf zenith-v1.0.0-linux-x86_64.tar.gz
sudo mv zenith /usr/local/bin/
```

**Windows:**
```powershell
# Add the extracted directory to your PATH
```

#### Method 4: Build from Source

```bash
git clone https://github.com/user/zenith.git
cd zenith
cargo build --release
sudo cp target/release/zenith /usr/local/bin/
```

### Verify Installation

```bash
zenith --version
# Output: zenith 1.0.1

zenith --help
# Show all available commands
```

---

## Quick Start

### Basic Usage

**Format a single file:**
```bash
zenith format src/main.rs
```

**Format a directory:**
```bash
zenith format src/
```

**Format recursively:**
```bash
zenith format ./ --recursive
```

**Check mode (no changes):**
```bash
zenith format src/ --check
```

### First Time Setup

1. **Initialize configuration:**
```bash
zenith init
```

2. **Check environment:**
```bash
zenith doctor
```

3. **Format your project:**
```bash
zenith format ./ --recursive
```

---

## Configuration

### Configuration File Location

Zenith looks for configuration files in the following order (first found wins):

1. `./zenith.toml` (current directory)
2. `~/.config/zenith/config.toml` (user config)
3. `/etc/zenith/config.toml` (system config)

### Configuration File Format

Create `zenith.toml`:

```toml
[global]
backup_enabled = true
log_level = "info"
recursive = false
cache_enabled = true

[zeniths.rust]
enabled = true
config_path = ".rustfmt.toml"

[zeniths.python]
enabled = true
config_path = "pyproject.toml"

[zeniths.javascript]
enabled = true
config_path = ".prettierrc"

[zeniths.json]
enabled = true

[concurrency]
workers = 8
batch_size = 100

[backup]
dir = ".zenith_backup"
retention_days = 7
max_backups = 50

[limits]
max_file_size_mb = 10

[mcp]
enabled = false
host = "127.0.0.1"
port = 8080
auth_enabled = false
api_key = null
allowed_origins = ["*"]

[[mcp.users]]
api_key = "your-secret-key"
role = "admin"
```

### Environment Variables

Override configuration with environment variables:

```bash
export ZENITH_WORKERS=16
export ZENITH_LOG_LEVEL=debug
export ZENITH_NO_BACKUP=false
export ZENITH_RECURSIVE=true
export ZENITH_CACHE_ENABLED=true
export ZENITH_BACKUP_DIR=".zenith_backup"
export ZENITH_MCP_ENABLED=true
```

### Formatter-Specific Configuration

#### Rust (rustfmt)

Create `.rustfmt.toml`:
```toml
max_width = 100
hard_tabs = false
tab_spaces = 4
edition = "2021"
```

#### Python (ruff/black)

Create `pyproject.toml`:
```toml
[tool.ruff]
line-length = 88
target-version = "py38"

[tool.black]
line-length = 88
target-version = ['py38']
```

#### JavaScript/TypeScript (prettier)

Create `.prettierrc`:
```json
{
  "semi": true,
  "singleQuote": false,
  "tabWidth": 2,
  "trailingComma": "es5"
}
```

#### C/C++ (clang-format)

Create `.clang-format`:
```yaml
BasedOnStyle: Google
IndentWidth: 2
ColumnLimit: 100
```

#### Java (google-java-format)

No configuration needed - uses Google Java Style.

#### JSON/YAML/TOML (Built-in)

No external configuration needed.

---

## Commands Reference

### format

Format files and directories.

**Syntax:**
```bash
zenith format [OPTIONS] <PATH>...
```

**Options:**
- `-r, --recursive`: Process directories recursively
- `-c, --check`: Check mode (no changes, exit 1 if changes needed)
- `--no-backup`: Disable backup for this run
- `-w, --workers <N>`: Number of concurrent workers
- `-v, --verbose`: Verbose output

**Examples:**
```bash
zenith format src/main.rs
zenith format src/ --recursive
zenith format ./ --check
zenith format src/ --workers 16 --verbose
```

### recover

Restore files from a backup.

**Syntax:**
```bash
zenith recover [OPTIONS] <BACKUP_ID>
```

**Options:**
- `-t, --target <DIR>`: Target directory for recovery (default: original location)
- `-v, --verbose`: Verbose output

**Examples:**
```bash
zenith recover backup_20231223_142030
zenith recover backup_20231223_142030 --target /tmp/restore
```

### list-backups

List all available backups.

**Syntax:**
```bash
zenith list-backups [OPTIONS]
```

**Options:**
- `--limit <N>`: Limit number of backups shown
- `--sort <FIELD>`: Sort by field (date, size, files)

**Examples:**
```bash
zenith list-backups
zenith list-backups --limit 10 --sort date
```

### clean-backups

Remove expired backups.

**Syntax:**
```bash
zenith clean-backups [OPTIONS]
```

**Options:**
- `-d, --days <N>`: Remove backups older than N days
- `--all`: Remove all backups
- `--dry-run`: Show what would be deleted without deleting

**Examples:**
```bash
zenith clean-backups --days 7
zenith clean-backups --dry-run
```

### mcp

Start the MCP server.

**Syntax:**
```bash
zenith mcp [OPTIONS]
```

**Options:**
- `--host <ADDR>`: Server host address (default: 127.0.0.1)
- `-p, --port <PORT>`: Server port (default: 8080)
- `--no-auth`: Disable authentication

**Examples:**
```bash
zenith mcp
zenith mcp --host 0.0.0.0 --port 9000
```

### doctor

Check system environment and dependencies.

**Syntax:**
```bash
zenith doctor
```

**Output:**
```
Checking system dependencies...
✓ rustfmt 1.5.1
✓ ruff 0.1.9
✓ prettier 3.1.0
✓ clang-format 15.0.7
✓ shfmt 3.7.0

All required tools are available.
```

### init

Initialize configuration file.

**Syntax:**
```bash
zenith init [OPTIONS]
```

**Options:**
- `-f, --force`: Overwrite existing configuration

**Examples:**
```bash
zenith init
zenith init --force
```

---

## Advanced Usage

### CI/CD Integration

**GitHub Actions:**
```yaml
name: Format Check

on: [push, pull_request]

jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Zenith
        run: curl -sSL https://raw.githubusercontent.com/user/zenith/main/install.sh | sh
      - name: Check formatting
        run: zenith format ./ --recursive --check
```

**GitLab CI:**
```yaml
format:
  stage: test
  script:
    - curl -sSL https://raw.githubusercontent.com/user/zenith/main/install.sh | sh
    - zenith format ./ --recursive --check
```

### Pre-commit Hooks

**Using pre-commit:**
```yaml
repos:
  - repo: local
    hooks:
      - id: zenith
        name: Zenith Format
        entry: zenith format
        language: system
        pass_filenames: true
```

**Using Git hooks directly:**
```bash
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
zenith format $(git diff --cached --name-only --diff-filter=ACM | grep -E '\.(rs|py|js|ts|json|yaml|yml|toml|md|sh)$')
git add -u
EOF
chmod +x .git/hooks/pre-commit
```

### Git Integration

**Format staged files:**
```bash
zenith format $(git diff --cached --name-only --diff-filter=ACM)
```

**Format changed files:**
```bash
zenith format $(git diff --name-only --diff-filter=ACM)
```

**Format files in last commit:**
```bash
zenith format $(git diff HEAD~1 --name-only)
```

### Performance Tuning

**Increase worker count for large projects:**
```bash
zenith format ./ --recursive --workers 32
```

**Batch processing:**
```toml
[concurrency]
workers = 16
batch_size = 200
```

**Disable cache for one-off formatting:**
```bash
ZENITH_CACHE_ENABLED=false zenith format ./ --recursive
```

### Selective Formatting

**Format specific file types:**
```bash
zenith format $(find ./ -name "*.rs" -o -name "*.py")
```

**Exclude directories:**
```bash
zenith format ./ --recursive --exclude target --exclude node_modules
```

---

## MCP Server

### Starting the Server

**Basic:**
```bash
zenith mcp
```

**Custom host/port:**
```bash
zenith mcp --host 0.0.0.0 --port 9000
```

### Authentication

Configure users in `zenith.toml`:
```toml
[mcp]
enabled = true
auth_enabled = true

[[mcp.users]]
api_key = "admin-secret-key"
role = "admin"

[[mcp.users]]
api_key = "user-secret-key"
role = "user"
```

### API Methods

#### format

Format files.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "format",
  "params": {
    "paths": ["/path/to/file.rs"],
    "recursive": false,
    "backup": true,
    "workers": 8
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "total_files": 1,
    "formatted_files": 1,
    "failed_files": 0,
    "backup_id": "backup_20231223_142030",
    "duration_ms": 150,
    "results": [
      {
        "path": "/path/to/file.rs",
        "success": true,
        "changed": true,
        "error": null
      }
    ]
  }
}
```

#### recover

Restore from backup.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "recover",
  "params": {
    "backup_id": "backup_20231223_142030",
    "target": "/path/to/restore"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "restored_files": 5,
    "duration_ms": 200
  }
}
```

### Client Example

**curl:**
```bash
curl -X POST http://127.0.0.1:8080 \
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

**Python:**
```python
import requests

response = requests.post(
    "http://127.0.0.1:8080",
    headers={
        "Authorization": "Bearer your-api-key",
        "Content-Type": "application/json"
    },
    json={
        "jsonrpc": "2.0",
        "id": 1,
        "method": "format",
        "params": {
            "paths": ["src/main.rs"],
            "recursive": False,
            "backup": True
        }
    }
)

print(response.json())
```

---

## Troubleshooting

### Common Issues

**Issue: Formatter not found**

```bash
zenith doctor
# Install missing formatters:
rustup component add rustfmt
pip install ruff
npm install -g prettier
```

**Issue: Permission denied**

```bash
# Use sudo or fix permissions
sudo zenith format /etc/config/file.conf
# Or:
chmod +w /path/to/file
```

**Issue: Backup directory full**

```bash
# Clean old backups
zenith clean-backups --days 7
```

**Issue: Slow performance**

```bash
# Increase workers
zenith format ./ --recursive --workers 32

# Or disable cache temporarily
ZENITH_CACHE_ENABLED=false zenith format ./ --recursive
```

### Debug Mode

Enable verbose logging:
```bash
zenith format ./ --recursive --verbose
```

Or set log level:
```bash
ZENITH_LOG_LEVEL=debug zenith format ./ --recursive
```

### Getting Help

```bash
zenith --help
zenith format --help
zenith recover --help
```

---

## Best Practices

### Project Setup

1. **Initialize configuration:**
```bash
zenith init
```

2. **Set up formatter configs:**
```bash
# Rust
echo "max_width = 100" > .rustfmt.toml

# Python
echo "[tool.ruff]" > pyproject.toml
echo "line-length = 88" >> pyproject.toml

# JavaScript
echo '{"semi": true}' > .prettierrc
```

3. **Add to CI:**
```bash
# Add format check to CI pipeline
zenith format ./ --recursive --check
```

### Workflow

1. **Development:**
```bash
# Work on your code
vim src/main.rs

# Format before commit
zenith format ./ --recursive
git add .
git commit -m "feat: add new feature"
```

2. **Review:**
```bash
# Check format in PR
zenith format ./ --recursive --check
```

3. **Recovery:**
```bash
# If something goes wrong
zenith list-backups
zenith recover backup_20231223_142030
```

### Performance Tips

- Use `--recursive` for large projects
- Adjust `--workers` based on CPU cores
- Enable cache for repeated formatting
- Use `--check` in CI pipelines

### Safety Tips

- Always keep backups enabled
- Review changes before committing
- Use `--check` in CI to catch issues
- Test recovery process occasionally

---

## Additional Resources

- [GitHub Repository](https://github.com/user/zenith)
- [Issue Tracker](https://github.com/user/zenith/issues)
- [Discussions](https://github.com/user/zenith/discussions)
- [Contributing Guide](docs/CONTRIBUTING.md)
