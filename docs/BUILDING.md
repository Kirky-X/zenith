# Building Zenith

This document provides comprehensive instructions for building Zenith on different platforms.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Quick Start](#quick-start)
3. [Building for Different Platforms](#building-for-different-platforms)
4. [Cross-Platform Builds](#cross-platform-builds)
5. [CI/CD Builds](#cicd-builds)
6. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Common Requirements

- **Rust**: 1.75 or later
- **Git**: For cloning the repository
- **C Compiler**: For compiling native dependencies

### Platform-Specific Requirements

#### Linux

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y build-essential git curl

# Fedora/RHEL
sudo dnf install -y gcc git curl
```

#### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Or using Homebrew
brew install git
```

#### Windows

1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
2. Install [Git for Windows](https://git-scm.com/download/win)
3. Install [Rustup](https://rustup.rs/)

---

## Quick Start

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/user/zenith.git
cd zenith

# Build the project
cargo build --release

# Run the binary
./target/release/zenith --help
```

### Development Build

```bash
# Build with debug symbols
cargo build

# Run tests
cargo test

# Run with cargo
cargo run -- format src/
```

---

## Building for Different Platforms

### Linux

#### x86_64 (Default)

```bash
# Standard build
cargo build --release

# Output: target/x86_64-unknown-linux-gnu/release/zenith
```

#### ARM64 (aarch64)

```bash
# Add ARM64 target
rustup target add aarch64-unknown-linux-gnu

# Install cross-compilation tools
sudo apt-get install gcc-aarch64-linux-gnu

# Build for ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Output: target/aarch64-unknown-linux-gnu/release/zenith
```

#### musl (Static Binary)

```bash
# Add musl target
rustup target add x86_64-unknown-linux-musl

# Install musl tools
sudo apt-get install musl-tools

# Build static binary
cargo build --release --target x86_64-unknown-linux-musl

# Output: target/x86_64-unknown-linux-musl/release/zenith
```

### macOS

#### Intel (x86_64)

```bash
# Build for Intel
cargo build --release

# Output: target/x86_64-apple-darwin/release/zenith
```

#### Apple Silicon (ARM64)

```bash
# Build for Apple Silicon
cargo build --release

# Output: target/aarch64-apple-darwin/release/zenith
```

#### Universal Binary (Intel + ARM64)

```bash
# Build both architectures
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary using lipo
lipo -create \
  target/x86_64-apple-darwin/release/zenith \
  target/aarch64-apple-darwin/release/zenith \
  -output target/release/zenith-universal
```

### Windows

#### MSVC (Recommended)

```bash
# Build with MSVC
cargo build --release

# Output: target/x86_64-pc-windows-msvc/release/zenith.exe
```

#### GNU (MinGW)

```bash
# Add GNU target
rustup target add x86_64-pc-windows-gnu

# Build with MinGW
cargo build --release --target x86_64-pc-windows-gnu

# Output: target/x86_64-pc-windows-gnu/release/zenith.exe
```

---

## Cross-Platform Builds

### Using cargo-cross

[cargo-cross](https://github.com/cross-rs/cross) is a tool for cross-compilation using Docker.

#### Installation

```bash
cargo install cross
```

#### Usage

```bash
# Build for Linux ARM64
cross build --release --target aarch64-unknown-linux-gnu

# Build for Windows
cross build --release --target x86_64-pc-windows-msvc

# Build for macOS (requires macOS SDK)
cross build --release --target aarch64-apple-darwin
```

### Using the Build Script

The project includes a comprehensive build script for cross-platform builds.

```bash
# Make the script executable
chmod +x scripts/build_cross_platform.sh

# Build for all available platforms
./scripts/build_cross_platform.sh

# Use cargo-cross for cross-compilation
./scripts/build_cross_platform.sh --cross

# Build for all platforms (requires Docker)
./scripts/build_cross_platform.sh --all --cross

# Skip platforms with missing toolchains
./scripts/build_cross_platform.sh --skip-missing

# Show help
./scripts/build_cross_platform.sh --help
```

#### Build Script Options

| Option | Description |
|--------|-------------|
| `--cross` | Use cargo-cross for cross-compilation |
| `--all` | Build for all supported platforms |
| `--skip-missing` | Skip platforms with missing toolchains |
| `--help` | Show help message |

#### Supported Platforms

| Target | Platform | Description |
|--------|----------|-------------|
| `x86_64-unknown-linux-gnu` | Linux x86_64 | Standard Linux build |
| `aarch64-unknown-linux-gnu` | Linux ARM64 | ARM64 Linux build |
| `x86_64-unknown-linux-musl` | Linux x86_64 (musl) | Static Linux binary |
| `aarch64-unknown-linux-musl` | Linux ARM64 (musl) | Static ARM64 binary |
| `x86_64-pc-windows-gnu` | Windows x86_64 (GNU) | Windows with MinGW |
| `x86_64-pc-windows-msvc` | Windows x86_64 (MSVC) | Windows with MSVC |
| `x86_64-apple-darwin` | macOS Intel | Intel Mac build |
| `aarch64-apple-darwin` | macOS Apple Silicon | Apple Silicon build |

---

## CI/CD Builds

The project uses GitHub Actions for automated builds.

### Workflow Triggers

- **Push to main/master**: Runs tests and builds
- **Pull Request**: Runs tests, linting, and builds
- **Tag (v*)**: Creates release with cross-platform binaries

### Build Jobs

#### Test Job

```yaml
- Runs on Ubuntu with Rust 1.75.0 and stable
- Executes all tests
- Runs integration tests
```

#### Lint Job

```yaml
- Runs formatting checks (rustfmt)
- Runs clippy linter
- Enforces code quality standards
```

#### Build Job

```yaml
- Builds on Ubuntu, Windows, and macOS
- Uses matrix strategy for parallel builds
- Caches dependencies for faster builds
```

#### Release Job

```yaml
- Triggered on version tags (v*)
- Builds for all supported platforms
- Creates GitHub release
- Uploads release assets
```

### Manual Trigger

To trigger a release build:

```bash
# Create and push a version tag
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

---

## Troubleshooting

### Common Issues

#### "linker not found" Error

**Problem**: Cross-compilation fails with linker errors.

**Solution**: Install the appropriate cross-compilation toolchain.

```bash
# For ARM64 on Linux
sudo apt-get install gcc-aarch64-linux-gnu

# For Windows on Linux
sudo apt-get install gcc-mingw-w64

# For musl
sudo apt-get install musl-tools
```

#### "target not installed" Error

**Problem**: Rust target is not installed.

**Solution**: Add the target using rustup.

```bash
rustup target add <target>
```

#### Docker Issues with cargo-cross

**Problem**: cargo-cross fails with Docker errors.

**Solution**: Ensure Docker is running and accessible.

```bash
# Check Docker status
docker ps

# Restart Docker if needed
sudo systemctl restart docker
```

#### Build Fails on macOS

**Problem**: Build fails on macOS with code signing errors.

**Solution**: Disable code signing for development builds.

```bash
export CODESIGN_ALLOCATE=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/codesign_allocate
```

#### Memory Issues During Build

**Problem**: Build fails with out-of-memory errors.

**Solution**: Limit the number of parallel jobs.

```bash
# Limit to 2 jobs
CARGO_BUILD_JOBS=2 cargo build --release
```

### Getting Help

If you encounter issues not covered here:

1. Check the [GitHub Issues](https://github.com/user/zenith/issues)
2. Search for similar issues
3. Create a new issue with:
   - Platform and OS version
   - Rust version (`rustc --version`)
   - Full error message
   - Steps to reproduce

---

## Build Optimization

### Release Profile

The project uses an optimized release profile in `Cargo.toml`:

```toml
[profile.release]
lto = true           # Link-time optimization
strip = true         # Strip debug symbols
opt-level = 3        # Maximum optimization
```

### Additional Optimization Options

```bash
# Build with maximum optimization
cargo build --release --features optimize

# Build with debug symbols (for profiling)
cargo build --release --debug

# Build with specific features
cargo build --release --features "mcp,backup"
```

### Caching Dependencies

Cargo caches dependencies automatically. To clear the cache:

```bash
# Clean build artifacts
cargo clean

# Clear dependency cache
cargo cache --remove-dir all
```

---

## Verifying Builds

### Check Binary

```bash
# Verify the binary was built
ls -lh target/release/zenith

# Check binary information
file target/release/zenith

# Run the binary
./target/release/zenith --version
```

### Run Tests

```bash
# Run all tests
cargo test --release

# Run specific test
cargo test --release test_name

# Run with output
cargo test --release -- --nocapture
```

### Benchmark Performance

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench benchmark_name
```

---

## Packaging for Distribution

### Creating Release Packages

#### Linux/macOS (tar.gz)

```bash
# Create tarball
tar -czf zenith-1.0.0-linux-x86_64.tar.gz \
  -C target/x86_64-unknown-linux-gnu/release \
  zenith

# Create checksums
sha256sum zenith-1.0.0-linux-x86_64.tar.gz > checksums.txt
```

#### Windows (zip)

```bash
# Create zip file
powershell Compress-Archive -Path target/x86_64-pc-windows-msvc/release/zenith.exe -DestinationPath zenith-1.0.0-windows-x86_64.zip

# Create checksums
certutil -hashfile zenith-1.0.0-windows-x86_64.zip SHA256 > checksums.txt
```

### Installation Script

The project includes an installation script (`scripts/install.sh`) for easy installation.

```bash
# Install latest version
./scripts/install.sh

# Install specific version
./scripts/install.sh --version 1.0.0

# Install to custom directory
./scripts/install.sh --dir /opt/zenith

# Install from local file
./scripts/install.sh --file ./zenith-x86_64-linux-gnu
```

---

## Development Workflow

### Recommended Workflow

1. **Clone the repository**
   ```bash
   git clone https://github.com/user/zenith.git
   cd zenith
   ```

2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature
   ```

3. **Make changes and test**
   ```bash
   cargo build
   cargo test
   ```

4. **Format and lint**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   ```

5. **Build release**
   ```bash
   cargo build --release
   ```

6. **Commit and push**
   ```bash
   git add .
   git commit -m "feat: add your feature"
   git push origin feature/your-feature
   ```

---

## Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [cargo-cross Documentation](https://github.com/cross-rs/cross)
