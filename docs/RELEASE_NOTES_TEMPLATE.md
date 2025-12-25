# Release Notes Template

Use this template when creating GitHub releases.

---

## [VERSION] - YYYY-MM-DD

### 🎉 Highlights

- Brief description of the most important changes
- Key features or improvements
- Any breaking changes

### ✨ Added

- New feature 1
- New feature 2
- ...

### 🐛 Fixed

- Bug fix 1
- Bug fix 2
- ...

### 🔄 Changed

- Change 1
- Change 2
- ...

### ⚡ Performance

- Performance improvement 1
- Performance improvement 2
- ...

### 📚 Documentation

- Documentation update 1
- Documentation update 2
- ...

### 🧹 Refactoring

- Refactoring 1
- Refactoring 2
- ...

### 🛠️ Developer Experience

- Developer experience improvement 1
- Developer experience improvement 2
- ...

### 🧪 Testing

- Test improvement 1
- Test improvement 2
- ...

### 📦 Dependencies

- Dependency update 1
- Dependency update 2
- ...

### 🔒 Security

- Security fix 1
- Security fix 2
- ...

### ⚠️ Breaking Changes

> **Important:** This release contains breaking changes. Please review carefully.

- Breaking change 1
  - Migration guide
- Breaking change 2
  - Migration guide

### 🚀 Upgrade Instructions

#### From Previous Version

\`\`\`bash
# Using install script
./scripts/install.sh --version VERSION

# Using cargo
cargo install zenith --version VERSION

# Or download from releases page
# https://github.com/user/zenith/releases/tag/vVERSION
\`\`\`

#### Migration Guide

If you have breaking changes, include migration steps here.

### 📥 Download

Choose the appropriate package for your platform:

| Platform | Download | Checksum |
|----------|----------|----------|
| Linux x86_64 | `zenith-VERSION-linux-x86_64.tar.gz` | SHA256 |
| Linux ARM64 | `zenith-VERSION-linux-arm64.tar.gz` | SHA256 |
| macOS Intel | `zenith-VERSION-macos-x86_64.tar.gz` | SHA256 |
| macOS Apple Silicon | `zenith-VERSION-macos-arm64.tar.gz` | SHA256 |
| Windows x86_64 | `zenith-VERSION-windows-x86_64-msvc.zip` | SHA256 |
| Source | `zenith-VERSION-source.tar.gz` | SHA256 |

### ✅ Verification

Verify the integrity of downloaded files:

\`\`\`bash
# Linux/macOS
sha256sum -c checksums.txt

# Windows
certutil -hashfile <file> SHA256
\`\`\`

### 📖 Documentation

- [User Guide](https://github.com/user/zenith/blob/main/docs/USE_GUIDE.md)
- [Developer Guide](https://github.com/user/zenith/blob/main/docs/CONTRIBUTING.md)
- [Building Instructions](https://github.com/user/zenith/blob/main/docs/BUILDING.md)
- [API Documentation](https://docs.rs/zenith)

### 🤝 Contributing

Thanks to all contributors who made this release possible!

- @contributor1 - Contribution description
- @contributor2 - Contribution description

### 📋 Full Changelog

For a complete list of changes, see the [CHANGELOG.md](https://github.com/user/zenith/blob/main/docs/CHANGELOG.md).

---

## Previous Releases

### [1.0.0] - 2024-12-24

Initial release of Zenith!

#### Features

- Multi-language code formatting support (Rust, Python, JavaScript, TypeScript, Go, Java, C, C++)
- Backup and recovery functionality
- MCP (Model Context Protocol) server integration
- Concurrent file processing with configurable parallelism
- Hash-based change detection for efficient formatting
- Cross-platform support (Linux, Windows, macOS)

#### Commands

- `zenith format` - Format code files
- `zenith recover` - Restore files from backups
- `zenith clean-backups` - Manage backup files
- `zenith doctor` - Diagnose system environment
- `zenith mcp` - Start MCP server
- `zenith init` - Initialize configuration

#### Documentation

- Complete user guide
- Developer contribution guide
- Building instructions
- API documentation

#### Download

Available for Linux, macOS, and Windows from the releases page.

---

## Release Checklist

Use this checklist when preparing a release:

- [ ] All tests passing
- [ ] Code coverage meets requirements
- [ ] Documentation updated
- [ ] docs/CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Release notes written
- [ ] Build artifacts created
- [ ] Checksums generated
- [ ] Binaries tested on all platforms
- [ ] Tag created (`git tag -a vVERSION -m "Release vVERSION"`)
- [ ] Tag pushed (`git push origin vVERSION`)
- [ ] GitHub release created
- [ ] Release assets uploaded
- [ ] Published to crates.io (if applicable)
- [ ] Announcement posted
