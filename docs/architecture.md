# Architecture Overview

PMX follows a modular architecture with clear separation of concerns. This document describes the high-level system design and component relationships.

## System Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CLI Interface │    │  Command Router │    │   Commands      │
│   (cli.rs)      │───▶│   (main.rs)     │───▶│   (commands/)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                  │                       │
                                  ▼                       ▼
                       ┌─────────────────┐    ┌─────────────────┐
                       │  Storage System │    │   File System   │
                       │  (storage.rs)   │───▶│   Operations    │
                       └─────────────────┘    └─────────────────┘
```

## Core Components

### 1. CLI Interface (`src/cli.rs`)

The command-line interface layer built on clap with derive macros.

**Key structures:**
- `Arg` - Main CLI parser with global options
- `Command` - Enum of all available commands
- `ProfileCommand` - Subcommands for profile management
- `InternalCompletionCommand` - Hidden completion helpers

**Code reference:** [src/cli.rs:5-94](../src/cli.rs#L5-L94)

### 2. Command Router (`src/main.rs`)

Central dispatch logic that handles storage initialization and command routing.

**Key responsibilities:**
- Parse CLI arguments using `clap::Parser`
- Initialize storage with auto-discovery fallback
- Route commands to appropriate handlers
- Handle global configuration options

**Storage initialization flow:**
```rust
// src/main.rs:8-12
let storage = args
    .config
    .or_else(|| std::env::var("PMX_CONFIG_FILE").ok().map(PathBuf::from))
    .map(pmx::storage::Storage::new)
    .unwrap_or_else(pmx::storage::Storage::auto)?;
```

### 3. Storage System (`src/storage.rs`)

Core data management layer with auto-discovery and validation.

**Key structures:**
- `Storage` - Main storage interface
- `Config` - TOML configuration structure
- `Agents` - Agent enable/disable flags

**Directory structure expected:**
```
$XDG_CONFIG_HOME/pmx/ (or ~/.config/pmx/)
├── config.toml           # Agent configuration
└── repo/                 # Profile storage
    ├── profile1.md
    ├── category/
    │   └── profile2.md
    └── ...
```

**Auto-discovery flow:**
```rust
// src/storage.rs:189-203
pub fn auto() -> crate::Result<Self> {
    let xdg_data_home = std::env::var("XDG_CONFIG_HOME").ok();
    let other_path = crate::utils::home_dir()
        .map(|p| p.join(".config/pmx"))
        .expect("Failed to get home directory");

    let path = xdg_data_home
        .map(PathBuf::from)
        .unwrap_or_else(|| other_path.clone());

    Self::new(path).or_else(|e| {
        eprintln!("Failed to load storage from {:?}: {}", other_path, e);
        Self::initialize(other_path)
    })
}
```

### 4. Commands Module (`src/commands/`)

Modular command implementations split by functionality.

#### 4.1 Claude Code Commands (`src/commands/claude_code.rs`)

Manages Claude profile operations targeting `~/.claude/CLAUDE.md`.

**Functions:**
- `set_claude_profile()` - Copy profile to Claude directory
- `reset_claude_profile()` - Remove current Claude profile

**Code reference:** [src/commands/claude_code.rs:3-36](../src/commands/claude_code.rs#L3-L36)

#### 4.2 Codex Commands (`src/commands/openai_codex.rs`)

Manages Codex profile operations targeting `~/.codex/AGENTS.md`.

**Functions:**
- `set_codex_profile()` - Copy profile to Codex directory  
- `reset_codex_profile()` - Remove current Codex profile

#### 4.3 Profile Management (`src/commands/profile.rs`)

Core CRUD operations for profile lifecycle management.

**Functions:**
- `create()` - Create new profile with editor
- `edit()` - Edit existing profile  
- `delete()` - Delete profile with confirmation
- `show()` - Display profile content
- `copy()` - Copy profile to clipboard

**Profile validation:**
```rust
// src/commands/profile.rs:175-217
fn validate_profile_name(name: &str) -> crate::Result<()> {
    // Length validation
    // Path traversal protection  
    // Invalid character filtering
    // Path component validation
}
```

#### 4.4 Utility Commands (`src/commands/utils.rs`)

Shared functionality across different command types.

**Functions:**
- `list()` - Tree-structured profile listing
- `copy_profile()` - Clipboard integration
- `completion()` - Shell completion generation
- `internal_completion()` - Dynamic completion data

**Tree output example:**
```
├── development/
│   ├── rust
│   └── python
├── design/
│   └── ui-ux
└── general
```

## Data Flow

### Profile Application Flow

1. **Command Parsing** - CLI arguments parsed by clap
2. **Storage Loading** - Auto-discover or load specified storage
3. **Profile Resolution** - Validate profile exists in storage
4. **Agent Check** - Verify agent not disabled in config
5. **File Operations** - Copy profile to target location
6. **Feedback** - Success/error message to user

### Profile Creation Flow

1. **Name Validation** - Security and format checks
2. **Existence Check** - Prevent overwriting existing profiles
3. **Template Generation** - Create initial content
4. **Editor Launch** - Open in $EDITOR/$VISUAL
5. **Content Validation** - Check for meaningful content
6. **Storage** - Write to repo directory

## Configuration System

### TOML Configuration Format

```toml
[agents]
disable_claude = false
disable_codex = false
```

**Code reference:** [src/storage.rs:11-21](../src/storage.rs#L11-L21)

### Agent Enable/Disable Logic

Commands are conditionally available based on configuration:

```rust
// src/commands/claude_code.rs:4-7
ensure!(
    !storage.config.agents.disable_claude,
    "Claude profiles are disabled in the configuration."
);
```

## Security Considerations

### Path Traversal Protection

Profile names are validated to prevent directory traversal:

```rust
// src/commands/profile.rs:184-188
if name.contains("..") || name.contains('\\') {
    return Err(anyhow!(
        "Profile name cannot contain '..' or backslashes"
    ));
}
```

### File System Safety

- All file operations use validated paths
- Temporary files for editing operations
- Atomic operations where possible
- Clear error messages for failed operations

## Error Handling Strategy

The codebase uses `anyhow` for error handling with context:

```rust
// Example from src/commands/profile.rs:15-18
let status = Command::new(&editor)
    .arg(&profile_path)
    .status()
    .with_context(|| format!("Failed to execute editor: {}", editor))?;
```

## Testing Architecture

Each module includes comprehensive unit tests:

- **Isolated Testing** - Temporary directories for storage tests
- **Configuration Testing** - Agent enable/disable scenarios  
- **Validation Testing** - Profile name validation edge cases
- **Integration Testing** - End-to-end command workflows

**Example test setup:**
```rust
// src/commands/profile.rs:226-250
fn create_test_storage() -> (TempDir, crate::storage::Storage) {
    let temp_dir = TempDir::new().unwrap();
    // ... setup test environment
    let storage = crate::storage::Storage::new(temp_dir.path().to_path_buf()).unwrap();
    (temp_dir, storage)
}
```