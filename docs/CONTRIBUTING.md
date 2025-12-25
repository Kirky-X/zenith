# Contributing to Zenith

Thank you for your interest in contributing to Zenith! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Workflow](#development-workflow)
4. [Coding Standards](#coding-standards)
5. [Testing Guidelines](#testing-guidelines)
6. [Documentation](#documentation)
7. [Pull Request Process](#pull-request-process)
8. [Release Process](#release-process)

---

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code.

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what is best for the community
- Show empathy towards other community members

---

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Git
- A code editor (VS Code recommended)

### Setting Up Development Environment

1. **Fork and clone the repository:**
```bash
git clone https://github.com/YOUR_USERNAME/zenith.git
cd zenith
```

2. **Add upstream remote:**
```bash
git remote add upstream https://github.com/user/zenith.git
```

3. **Install development tools:**
```bash
# Install Rust toolchain
rustup install stable
rustup default stable

# Install development components
rustup component add rustfmt clippy

# Install pre-commit hooks (optional)
cargo install cargo-pre-commit
```

4. **Build the project:**
```bash
cargo build
```

5. **Run tests:**
```bash
cargo test
```

6. **Run the tool:**
```bash
cargo run -- format test.rs
```

---

## Development Workflow

### Branch Naming

Follow the Git workflow rules defined in the project:

**Format:** `<type>/<ticket-id>-<description>`

**Types:** `feature/`, `bugfix/`, `hotfix/`, `release/`, `refactor/`, `docs/`

**Examples:**
- `feature/USER-123-add-authentication`
- `bugfix/USER-456-fix-memory-leak`
- `docs/USER-789-update-readme`

### Commit Messages

Follow the conventional commit format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code formatting
- `refactor`: Refactoring
- `test`: Testing

**Example:**
```
feat(formatter): add support for Go language

Implement Go formatter using gofmt. Add tests for
Go file detection and formatting.

Closes #123
```

### Development Cycle

1. **Create a feature branch:**
```bash
git checkout -b feature/USER-123-add-feature
```

2. **Make your changes:**
```bash
# Edit files
# Write tests
# Update documentation
```

3. **Run checks locally:**
```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test

# Run benchmarks (if applicable)
cargo bench
```

4. **Commit your changes:**
```bash
git add .
git commit -m "feat: add new feature"
```

5. **Push to your fork:**
```bash
git push origin feature/USER-123-add-feature
```

6. **Create a Pull Request**

---

## Coding Standards

### Rust Style Guide

Follow the project's Rust style guide:

- Use `rustfmt` for automatic formatting
- Limit line width to 120 characters
- Use 4-space indentation

### Naming Conventions

- **Modules, functions, variables:** `snake_case`
- **Types, traits:** `PascalCase`
- **Constants, static variables:** `SCREAMING_SNAKE_CASE`
- **Lifetime parameters:** `'a`, `'b`

### Code Quality

- Enable `clippy::all` and `clippy::pedantic`
- Allow reasonable exceptions with `#[allow(clippy::...)]`
- Write descriptive variable and function names
- Add comments for complex logic

### Error Handling

- Use `Result<T, E>` for error handling
- Implement `std::error::Error` for custom errors
- Provide helpful error messages with context
- Never use `panic!` in library code

### Ownership and Borrowing

- Favor borrowing over ownership transfer
- Use `&` for immutable borrows and `&mut` for mutable borrows
- Avoid unnecessary `.clone()`
- Keep lifetime annotations simple

### Concurrency

- Use `Arc<T>` for sharing immutable data
- Use `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for sharing mutable data
- Favor channels for inter-thread communication
- Pay attention to `Send` and `Sync` trait constraints

### Performance

- Use `Vec::with_capacity()` to pre-allocate memory
- Prefer iterator chains over loops
- Use `&str` instead of `String` as function arguments
- Use `#[inline]` for small, performance-critical functions

---

## Testing Guidelines

### Test Coverage

- Aim for >80% test coverage
- Write unit tests for all public functions
- Write integration tests for major features
- Test edge cases and error conditions

### Test Organization

```
tests/
├── core_components_test.rs  # Core functionality tests
├── formatter_test.rs        # Formatter unit tests
├── integration_test.rs      # Integration tests
├── mcp_test.rs             # MCP interface tests
└── cross_platform_test.rs  # Cross-platform tests
```

### Writing Tests

**Unit tests (in src/):**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        let result = function_name();
        assert_eq!(result, expected_value);
    }

    #[test]
    fn test_error_case() {
        let result = function_that_fails();
        assert!(result.is_err());
    }
}
```

**Integration tests (in tests/):**
```rust
use zenith::config::types::AppConfig;

#[tokio::test]
async fn test_integration_scenario() {
    let config = AppConfig::default();
    let result = some_async_function(config).await;
    assert!(result.is_ok());
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_function_name

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode (faster)
cargo test --release
```

### Benchmarks

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench benchmark_name
```

---

## Documentation

### Code Documentation

- Document all public APIs with `///` doc comments
- Include examples in documentation
- Document module-level behavior with `//!` comments

**Example:**
```rust
/// Formats a file using the appropriate formatter.
///
/// # Arguments
///
/// * `path` - Path to the file to format
/// * `config` - Configuration options
///
/// # Returns
///
/// Returns `Ok(())` if formatting succeeded, `Err` otherwise.
///
/// # Examples
///
/// ```
/// use zenith::format_file;
/// let result = format_file("test.rs", &config);
/// assert!(result.is_ok());
/// ```
pub async fn format_file(path: &str, config: &Config) -> Result<()> {
    // Implementation
}
```

### API Documentation

Generate API documentation:
```bash
# Generate documentation
cargo doc --open

# Generate documentation for all dependencies
cargo doc --open --all-features
```

### User Documentation

- Keep README.md up to date
- Update USE_GUIDE.md when adding features
- Add examples for new functionality

### Architecture Documentation

- Update architecture diagrams when changing structure
- Document design decisions in ARCHITECTURE.md
- Keep task.md updated with current progress

---

## Pull Request Process

### Before Submitting

1. **Run all checks:**
```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

2. **Update documentation:**
   - Update README.md if needed
   - Update USE_GUIDE.md if adding user-facing features
   - Add/update inline code documentation

3. **Write a good PR description:**
   - Describe the change
   - Explain why it's needed
   - List related issues
   - Include screenshots if applicable

### PR Template

```markdown
## Change Type
- [ ] New Feature
- [ ] Bug Fix
- [ ] Refactor
- [ ] Documentation
- [ ] Performance

## Description
Brief description of purpose and content

## Testing
- [ ] Unit tests passed
- [ ] Integration tests passed
- [ ] Manual testing completed

## Checklist
- [ ] Follows project standards
- [ ] Added necessary tests
- [ ] Updated documentation
- [ ] No breaking changes (or documented)

## Related Issue
Closes #123
```

### Review Process

1. **Automated checks:**
   - CI must pass
   - Code coverage must not decrease
   - Clippy warnings must be addressed

2. **Code review:**
   - At least one approval required
   - Address all review comments
   - Keep PRs small and focused

3. **Merge:**
   - Squash merge for clean history
   - Delete feature branch after merge

---

## Release Process

### Versioning

Follow Semantic Versioning: `MAJOR.MINOR.PATCH`

- **MAJOR**: Incompatible API changes
- **MINOR**: Backward-compatible features
- **PATCH**: Backward-compatible fixes

### Release Steps

1. **Update version in Cargo.toml:**
```toml
[package]
name = "zenith"
version = "1.1.0"
```

2. **Update docs/CHANGELOG.md:**
```markdown
## [1.1.0] - 2024-01-15

### Added
- New feature A
- New feature B

### Fixed
- Bug fix C

### Changed
- Improvement D
```

3. **Tag the release:**
```bash
git tag -a v1.1.0 -m "Release v1.1.0"
git push origin v1.1.0
```

4. **Create GitHub Release:**
   - Go to Releases page
   - Draft new release
   - Attach release notes
   - Upload binaries

5. **Publish to crates.io:**
```bash
cargo publish
```

### Release Checklist

- [ ] All tests passing
- [ ] Documentation updated
- [ ] docs/CHANGELOG.md updated
- [ ] Version bumped
- [ ] Tag created
- [ ] GitHub release created
- [ ] Published to crates.io

---

## Getting Help

- **Documentation:** Check README.md and USE_GUIDE.md
- **Issues:** Search existing issues or create a new one
- **Discussions:** Ask questions in GitHub Discussions
- **Discord:** Join our Discord server (link in README)

---

## Recognition

Contributors will be recognized in:
- README.md contributors list
- Release notes
- Annual contributor highlights

Thank you for contributing to Zenith! 🎉
