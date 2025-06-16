# Development Guide

This guide provides information for developers who want to contribute to PMX or understand its development workflow.

## Table of Contents

- [Development Environment](#development-environment)
- [Project Structure](#project-structure)
- [Building and Testing](#building-and-testing)
- [Code Style and Conventions](#code-style-and-conventions)
- [Contributing](#contributing)
- [Release Process](#release-process)

## Development Environment

### Prerequisites

- **Rust:** Latest stable version (2024 edition)
- **Cargo:** Included with Rust installation
- **Git:** For version control

### Setup

```bash
# Clone the repository
git clone <repository-url>
cd pmx

# Build the project
cargo build

# Run tests
cargo test

# Check formatting and linting
cargo fmt --check
cargo clippy
```

### Dependencies

The project uses the following key dependencies:

```toml
[dependencies]
clap = { version = "4.5.39", features = ["derive"] }  # CLI parsing
anyhow = "1.0.98"                                     # Error handling
serde = { version = "1.0.219", features = ["derive"] } # Serialization
toml = "0.8.22"                                       # TOML config parsing
arboard = "3.2.0"                                     # Clipboard integration
dialoguer = "0.11.0"                                  # Interactive prompts
tempfile = "3.20.0"                                   # Temporary files
is-terminal = "0.4"                                   # Terminal detection

[dev-dependencies]
tempfile = "3.20.0"                                   # Test utilities
```

## Project Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root and type aliases
├── cli.rs               # Command-line interface definitions
├── storage.rs           # Storage and configuration management
├── utils.rs             # Cross-platform utilities
├── commands.rs          # Commands module declaration
└── commands/
    ├── claude_code.rs   # Claude profile management
    ├── openai_codex.rs  # Codex profile management
    ├── profile.rs       # Profile CRUD operations
    └── utils.rs         # Utility commands

completions/             # Shell completion files
├── _pmx                 # Zsh completion script

docs/                    # Documentation
├── README.md           # Documentation index
├── architecture.md     # System architecture
├── api.md             # API reference
├── user-guide.md      # User documentation
└── development.md     # This file
```

### Module Responsibilities

| Module | Responsibility | Key Functions |
|--------|----------------|---------------|
| `main.rs` | Application entry, command routing | `main()` |
| `cli.rs` | CLI structure, argument parsing | Command definitions |
| `storage.rs` | Data persistence, configuration | Storage CRUD, auto-discovery |
| `commands/claude_code.rs` | Claude integration | `set_claude_profile()`, `reset_claude_profile()` |
| `commands/openai_codex.rs` | Codex integration | `set_codex_profile()`, `reset_codex_profile()` |
| `commands/profile.rs` | Profile management | `create()`, `edit()`, `delete()`, `show()`, `copy()` |
| `commands/utils.rs` | Shared utilities | `list()`, `completion()`, `copy_profile()` |
| `utils.rs` | Cross-platform helpers | `home_dir()` |

## Building and Testing

### Development Commands

As specified in [CLAUDE.md](../CLAUDE.md):

```bash
# Build the project
cargo build

# Run the application
cargo run -- <command>

# Run tests
cargo test

# Check code formatting and linting
cargo fmt --check
cargo clippy

# Apply code formatting
cargo fmt
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test storage::tests

# Run specific test function
cargo test test_validate_profile_name_valid
```

### Test Structure

Tests are co-located with the modules they test:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    // Test helper functions
    // Individual test functions
}
```

**Common test patterns:**

1. **Temporary directories** for storage tests:
```rust
fn create_test_storage() -> (TempDir, crate::storage::Storage) {
    let temp_dir = TempDir::new().unwrap();
    // Setup test environment
    let storage = crate::storage::Storage::new(temp_dir.path().to_path_buf()).unwrap();
    (temp_dir, storage)
}
```

2. **Configuration testing** with different agent settings:
```rust
fn create_test_storage(disable_claude: bool, disable_codex: bool) -> (TempDir, Storage) {
    // Create storage with specific agent configuration
}
```

3. **Validation testing** for edge cases:
```rust
#[test]
fn test_validate_profile_name_invalid() {
    assert!(validate_profile_name("../invalid").is_err());
    assert!(validate_profile_name("invalid\\name").is_err());
    // ... more edge cases
}
```

## Code Style and Conventions

### Rust Conventions

- **Edition:** 2024
- **Formatting:** Use `cargo fmt` with default settings
- **Linting:** Address all `cargo clippy` warnings
- **Naming:** Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)

### Error Handling

- Use `anyhow::Result<T>` for all fallible functions
- Provide context with `anyhow::anyhow!()` or `.with_context()`
- Use `ensure!()` macro for validation checks

```rust
// Good error handling examples
pub fn example_function() -> crate::Result<()> {
    ensure!(condition, "Descriptive error message");
    
    operation()
        .map_err(|e| anyhow::anyhow!("Context about operation: {}", e))?;
    
    other_operation()
        .with_context(|| "Context about other operation")?;
    
    Ok(())
}
```

### Documentation

- Document all public functions with `///` comments
- Include examples for complex functions
- Use `#[doc = "..."]` for conditional documentation
- Reference code locations in documentation: `src/file.rs:line`

### Code Organization

1. **Imports:** Group by source (std, external crates, internal modules)
2. **Functions:** Public functions first, then private helpers
3. **Tests:** At the end of each module in `#[cfg(test)]` blocks
4. **Constants:** At the top of modules after imports

### Testing Guidelines

1. **Test naming:** Descriptive function names starting with `test_`
2. **Test coverage:** Cover happy path, error cases, and edge cases
3. **Test isolation:** Use temporary directories/files for filesystem tests
4. **Test data:** Create minimal test data that validates the specific behavior

## Contributing

### Workflow

1. **Fork** the repository
2. **Create** a feature branch from `main`
3. **Implement** your changes with tests
4. **Ensure** tests pass and code is formatted
5. **Submit** a pull request

### Pull Request Guidelines

- **Title:** Clear, descriptive summary of changes
- **Description:** Explain what and why, not just how
- **Tests:** Include tests for new functionality
- **Documentation:** Update relevant documentation
- **Breaking changes:** Clearly identify any breaking changes

### Code Review Checklist

- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] New functionality has tests
- [ ] Documentation is updated
- [ ] Error handling follows project patterns
- [ ] Security considerations addressed

## Release Process

### Version Management

PMX follows semantic versioning (SemVer):

- **MAJOR:** Breaking changes to CLI or API
- **MINOR:** New features, backwards compatible
- **PATCH:** Bug fixes, internal improvements

### Release Steps

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG** with release notes
3. **Create git tag** with version number
4. **Build release** binaries for distribution
5. **Publish** to crates.io (if applicable)

### Build Targets

Common build targets for releases:

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# macOS x86_64
cargo build --release --target x86_64-apple-darwin

# macOS ARM64
cargo build --release --target aarch64-apple-darwin

# Windows x86_64
cargo build --release --target x86_64-pc-windows-msvc
```

## Security Considerations

### Input Validation

- **Profile names:** Validate against path traversal and filesystem restrictions
- **File paths:** Use absolute paths and validate existence
- **Commands:** Sanitize all shell command inputs

### File Operations

- **Temporary files:** Use `tempfile` crate for secure temporary file handling
- **Permissions:** Respect existing file permissions
- **Atomic operations:** Use atomic file operations where possible

### Configuration

- **Default security:** Secure defaults in configuration
- **Path validation:** Validate all configuration paths
- **Environment variables:** Sanitize environment variable inputs

## Debugging

### Common Debug Scenarios

1. **Storage issues:**
```bash
# Check storage location and structure
ls -la ~/.config/pmx/
cat ~/.config/pmx/config.toml
```

2. **Profile operations:**
```bash
# Test profile listing
cargo run -- profile list

# Test profile creation with debug output
RUST_LOG=debug cargo run -- profile create test-profile
```

3. **Agent integration:**
```bash
# Test Claude profile application
cargo run -- set-claude-profile test-profile
ls -la ~/.claude/
```

### Logging

Add logging for debugging during development:

```rust
use log::{debug, info, warn, error};

debug!("Storage path: {:?}", storage.path);
info!("Profile created: {}", name);
warn!("Profile already exists: {}", name);
error!("Failed to create profile: {}", error);
```

Run with logging:
```bash
RUST_LOG=debug cargo run -- command
```

## Performance Considerations

### File System Operations

- **Batch operations:** Group multiple file operations when possible
- **Lazy loading:** Don't read all profiles unless necessary
- **Error handling:** Fail fast for invalid operations

### Memory Usage

- **Streaming:** Stream large files rather than loading entirely into memory
- **String handling:** Use `&str` where possible instead of `String`
- **Collections:** Use appropriate collection types for data access patterns

### Completion Performance

Shell completions should be fast for interactive use:

```rust
// Good: Quick profile listing
pub fn internal_completion(...) -> Result<()> {
    let profiles = storage.list_repos()?;  // Cached/efficient
    for profile in profiles {
        println!("{}", profile);
    }
}
```