# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

**Build the project:**
```bash
cargo build
```

**Run the application:**
```bash
cargo run -- <command>
```

**Run tests:**
```bash
cargo test
```

**Check code formatting and linting:**
```bash
cargo fmt --check
cargo clippy
```

**Apply code formatting:**
```bash
cargo fmt
```

## Architecture Overview

This is a Rust CLI tool (`pmx`) for managing Claude profile configurations. The application follows a modular architecture with clear separation of concerns:

### Core Components

- **CLI Interface** (`cli.rs`): Uses clap for command-line argument parsing with three main commands:
  - `list`: Shows available profiles
  - `set-claude-profile <path>`: Applies a profile to `~/.claude/CLAUDE.md`
  - `reset-claude-profile`: Removes the current Claude profile

- **Storage System** (`storage.rs`): Manages profile storage with automatic configuration discovery:
  - Looks for config in `$XDG_CONFIG_HOME` or `~/.config/pmx`
  - Auto-initializes storage directory with `repo/` subdirectory for profile files
  - Validates storage structure and config.toml presence
  - Tracks agent configuration (Claude, Codex, Cline enable/disable flags)

- **Commands** (`commands.rs`): Implements the business logic for each CLI operation:
  - Profile listing via recursive directory traversal
  - Profile application by copying `.md` files to `~/.claude/CLAUDE.md`
  - Profile reset by removing the active Claude configuration

### Key Design Patterns

The application uses a configuration-first approach where storage auto-discovery ensures the tool works without manual setup. The storage system expects a specific directory structure with `repo/` containing profile `.md` files and a `config.toml` for agent settings.

### Dependencies

- `clap`: Command-line argument parsing with derive macros
- `anyhow`: Error handling with context
- `serde`/`toml`: Configuration serialization
- `tempfile`: Test utilities (dev dependency)