# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

**Build the project:**
```bash
cargo build
cargo build --release  # For optimized release build
```

**Run the application:**
```bash
cargo run -- <command>
cargo run -- list  # Example: list all profiles
```

**Run tests:**
```bash
cargo test
cargo test -- --nocapture  # Show println! output
# Note: test_copy_existing_profile is skipped in CI
```

**Check code formatting and linting:**
```bash
cargo fmt --check
cargo clippy -- -D warnings  # Warnings treated as errors
```

**Apply code formatting:**
```bash
cargo fmt
```

**Generate documentation:**
```bash
cargo doc --no-deps
```

## Architecture Overview

This is a Rust CLI tool (`pmx`) for managing AI agent profile configurations (Claude, Codex, etc.). The application follows a modular architecture with clear separation of concerns and strong type safety.

### Core Components

- **CLI Interface** (`cli.rs`): Uses clap v4 for command-line argument parsing with these commands:
  - `list`: Shows available profiles in tree format (smart terminal detection)
  - `new <name>`: Create a new profile
  - `edit <name>`: Edit existing profile using $EDITOR
  - `delete <name>`: Delete a profile
  - `show <name>`: Display profile contents
  - `set-claude-profile <name>`: Apply profile to `~/.claude/CLAUDE.md`
  - `append-claude-profile <name>`: Append profile to existing Claude config
  - `reset-claude-profile`: Remove current Claude profile
  - `set-codex-profile <name>`: Apply profile to `~/.codex/AGENTS.md`
  - `append-codex-profile <name>`: Append profile to existing Codex config
  - `reset-codex-profile`: Remove current Codex profile
  - `copy-profile <name>`: Copy profile content to clipboard
  - `completion <shell>`: Generate shell completions
  - `mcp`: Run as an MCP (Model Context Protocol) server

- **Storage System** (`storage.rs`): Manages profile storage with automatic configuration discovery:
  - Config location priority: `$PMX_CONFIG_FILE` > `$XDG_CONFIG_HOME/pmx` > `~/.config/pmx`
  - Auto-initializes storage directory with `repo/` subdirectory for profile files
  - Validates storage structure and creates default `config.toml` if missing
  - Supports nested directory organization for profiles
  - Path traversal protection for security

- **Commands Module** (`commands/`): Modular command implementations:
  - `profile.rs`: CRUD operations for profiles (new, edit, delete, show)
  - `claude_code.rs`: Claude profile management with set/append/reset
  - `openai_codex.rs`: Codex profile management with set/append/reset
  - `utils.rs`: Shared utilities (list, copy-profile, completions)
  - `mcp.rs`: MCP server implementation using tokio async runtime

### Key Design Patterns

1. **Configuration-First Approach**: Auto-discovery ensures the tool works without manual setup
2. **Error Context**: All errors use `anyhow` with rich context via `.with_context()`
3. **Validation**: Input validation using `ensure!()` macro for early failure
4. **Path Safety**: Profile names validated against path traversal attacks
5. **Smart Output**: Terminal detection for appropriate output formatting (tree vs plain)
6. **Async Support**: MCP server uses tokio for async operations

### Dependencies and Their Purpose

```toml
clap = "4.5"          # CLI parsing with derive macros
anyhow = "1.0"        # Error handling with context
serde = "1.0"         # Serialization framework
toml = "0.8"          # TOML config parsing
arboard = "3.2"       # Cross-platform clipboard
dialoguer = "0.11"    # Interactive confirmations
tempfile = "3.20"     # Safe temporary files
is-terminal = "0.4"   # Terminal detection
rmcp = "0.2"          # MCP protocol support
tokio = "1"           # Async runtime for MCP
serde_json = "1.0"    # JSON for MCP messages
```

### Configuration Structure

**`config.toml` format:**
```toml
[agents]
disable_claude = false
disable_codex = false

[mcp]
disable_prompts = false  # Can be bool or array: ["prompt1", "prompt2"]
disable_tools = false    # Can be bool or array: ["tool1", "tool2"]
```

### Testing Approach

- Unit tests co-located with modules
- Integration tests using temporary directories via `tempfile`
- Test helper functions in `storage.rs` for common setup
- Environment-specific test exclusions (e.g., clipboard tests in CI)