# Cross-Platform Building Guide for Zenith

This document explains how to build Zenith for multiple platforms using the cross-compilation setup.

## Build Script

The `build_cross_platform.sh` script automatically detects available cross-compilation tools and builds for all supported targets. It provides clear instructions when tools are missing.

### Usage

```bash
# Basic build (builds for available targets on current platform)
./scripts/build_cross_platform.sh

# Build for all platforms using cargo-cross (requires Docker)
./scripts/build_cross_platform.sh --all --cross

# Use cargo-cross for cross-compilation
./scripts/build_cross_platform.sh --cross

# Skip platforms with missing toolchains
./scripts/build_cross_platform.sh --skip-missing

# Show help
./scripts/build_cross_platform.sh --help
```

### Options

- `--cross`: Use cargo-cross for cross-compilation (recommended for complex targets)
- `--all`: Build for all supported platforms (may require Docker)
- `--skip-missing`: Skip platforms with missing toolchains instead of failing
- `--help`: Show help message

## Supported Targets

- `x86_64-unknown-linux-gnu` - Linux x86_64 (GNU)
- `aarch64-unknown-linux-gnu` - Linux ARM64 (GNU)
- `x86_64-unknown-linux-musl` - Linux x86_64 (musl, static linking)
- `aarch64-unknown-linux-musl` - Linux ARM64 (musl, static linking)
- `x86_64-pc-windows-gnu` - Windows x86_64 (GNU)
- `x86_64-pc-windows-msvc` - Windows x86_64 (MSVC)
- `x86_64-apple-darwin` - macOS Intel
- `aarch64-apple-darwin` - macOS Apple Silicon

## Installing Cross-Compilation Tools

### Ubuntu/Debian
```bash
# For Linux ARM64 cross-compilation
sudo apt-get install gcc-aarch64-linux-gnu

# For Windows cross-compilation
sudo apt-get install gcc-mingw-w64

# For musl cross-compilation (static linking)
sudo apt-get install musl-tools
```

### CentOS/RHEL
```bash
# For Linux ARM64 cross-compilation
sudo yum install gcc-aarch64-linux-gnu

# For Windows cross-compilation
sudo yum install mingw64-gcc mingw32-gcc

# For musl cross-compilation
sudo yum install musl-gcc
```

### Using cargo-cross (Recommended)

For easier cross-compilation, use [cargo-cross](https://github.com/cross-rs/cross):

```bash
cargo install cross
```

cargo-cross uses Docker containers to provide isolated build environments for each target platform, eliminating the need to install cross-compilation tools manually.

## Platform-Specific Notes

### macOS Cross-Compilation

Cross-compiling for macOS from Linux requires special setup. The recommended approach is to:

1. Use cargo-cross for macOS cross-compilation
2. Run the build script on a macOS machine directly

### Windows Cross-Compilation

Windows builds can be done using:
- Native MSVC toolchain on Windows
- MinGW cross-compilation from Linux
- cargo-cross (recommended for consistency)

### Static Linking with musl

The `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` targets produce statically linked binaries that don't depend on system libraries, making them ideal for:
- Alpine Linux containers
- Embedded systems
- Portable deployments

## Building

### Quick Start

```bash
# Make the script executable
chmod +x scripts/build_cross_platform.sh

# Build for all available targets on current platform
./scripts/build_cross_platform.sh
```

### Build for All Platforms

```bash
# Build for all platforms using cargo-cross (requires Docker)
./scripts/build_cross_platform.sh --all --cross
```

### Build Output

Binaries and archives will be placed in the `dist/` directory:
- Windows: `zenith-x86_64-pc-windows-gnu.exe` or `zenith-x86_64-pc-windows-msvc.exe`
- Linux/macOS: `zenith-<target>.tar.gz`

The script provides a summary showing successful, failed, and skipped builds.

## CI/CD Integration

The GitHub Actions workflow (`.github/workflows/ci.yml`) automatically builds for all platforms on release tags. The workflow:
1. Installs cross-compilation tools
2. Installs cargo-cross
3. Adds all target platforms
4. Runs the build script with `--all --cross` flags
5. Uploads artifacts to the release

## Troubleshooting

### Missing Cross-Compilation Tools

If cross-compilation tools are not found, the script will:
- Display a warning message
- Provide installation instructions
- Skip the target (with `--skip-missing`) or fail (default)

### cargo-cross Issues

If cargo-cross fails:
- Ensure Docker is installed and running
- Try building without `--cross` flag
- Check Docker logs for specific errors

### Blake3 Compilation Issues

The project uses `blake3` with the `pure` feature to avoid C compilation issues during cross-compilation. If you encounter issues:
- Ensure the `pure` feature is enabled in `Cargo.toml`
- Use cargo-cross for consistent build environments

### macOS Linker Issues

The `.cargo/config.toml` file contains linker configuration for macOS targets. If you encounter linker errors:
- Verify the linker is installed
- Check that `MACOSX_DEPLOYMENT_TARGET` is set correctly
- Consider using cargo-cross for macOS builds

### Build Fails for Specific Target

If a specific target fails to build:
1. Check the error message for missing dependencies
2. Try building with `--skip-missing` to skip problematic targets
3. Use `--cross` flag for better cross-compilation support
4. Verify the target is supported in `.cargo/config.toml`