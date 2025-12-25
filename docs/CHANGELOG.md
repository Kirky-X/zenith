# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Zenith
- Multi-language code formatting support (Rust, Python, JavaScript, TypeScript, Go, Java, C, C++)
- Backup and recovery functionality
- MCP (Model Context Protocol) server integration
- Concurrent file processing with configurable parallelism
- Hash-based change detection for efficient formatting
- Cross-platform support (Linux, Windows, macOS)

### Features
- **Format Command**: Format single files or entire directories
- **Recover Command**: Restore files from backups
- **Clean-Backups Command**: Manage and clean backup files
- **Doctor Command**: Diagnose system environment and dependencies
- **MCP Command**: Start MCP server for AI assistant integration
- **Init Command**: Initialize configuration files

### Configuration
- TOML-based configuration system
- Per-language formatter settings
- Backup management options
- MCP server configuration with authentication

### Performance
- Concurrent file processing with configurable thread pool
- Hash-based change detection to skip unchanged files
- Efficient memory usage with batch processing
- Optimized for large codebases

### Documentation
- User guide (USE_GUIDE.md)
- Developer guide (CONTRIBUTING.md)
- Building instructions (BUILDING.md)
- API documentation

## [1.0.1] - 2024-12-24

### Fixed
- Fixed test timeout issues in MCP server tests
- Fixed compilation errors in test files
- Corrected McpConfig initialization in tests

### Changed
- Improved test coverage for formatters
- Enhanced error messages for better debugging

## [1.0.0] - 2024-12-24

### Added
- Initial release of Zenith
- Multi-language code formatting support
- Backup and recovery functionality
- MCP server integration
- Concurrent file processing
- Hash-based change detection
- Cross-platform support

[Unreleased]: https://github.com/user/zenith/compare/v1.0.1...HEAD
[1.0.1]: https://github.com/user/zenith/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/user/zenith/releases/tag/v1.0.0
