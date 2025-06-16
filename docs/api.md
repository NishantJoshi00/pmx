# API Reference

This document provides detailed API documentation for all modules and functions in the PMX codebase.

## Table of Contents

- [Core Types](#core-types)
- [CLI Module (`cli.rs`)](#cli-module-clirs)
- [Storage Module (`storage.rs`)](#storage-module-storagers)
- [Commands Module (`commands/`)](#commands-module-commands)
  - [Claude Code Commands](#claude-code-commands)
  - [Codex Commands](#codex-commands)
  - [Profile Commands](#profile-commands)
  - [Utility Commands](#utility-commands)
- [Utils Module (`utils.rs`)](#utils-module-utilsrs)

## Core Types

### Result Type Alias

```rust
pub(crate) type Result<T> = anyhow::Result<T>;
```

**Location:** [src/lib.rs:6](../src/lib.rs#L6)

A project-wide type alias for `anyhow::Result` providing consistent error handling across all modules.

## CLI Module (`cli.rs`)

### Main CLI Parser

#### `struct Arg`

```rust
#[derive(Parser, Debug)]
pub struct Arg {
    pub config: Option<PathBuf>,
    pub command: Command,
}
```

**Location:** [src/cli.rs:5-15](../src/cli.rs#L5-L15)

Main CLI parser with global options and command dispatch.

**Fields:**
- `config: Option<PathBuf>` - Optional path to storage directory (overrides auto-discovery)
- `command: Command` - The subcommand to execute

### Commands

#### `enum Command`

```rust
#[derive(Debug, Subcommand)]
pub enum Command {
    SetClaudeProfile(ClaudeProfile),
    ResetClaudeProfile,
    SetCodexProfile(CodexProfile),
    ResetCodexProfile,
    Profile(ProfileCommand),
    Completion(CompletionArgs),
    InternalCompletion(InternalCompletionCommand),
}
```

**Location:** [src/cli.rs:17-35](../src/cli.rs#L17-L35)

Top-level command enumeration supporting all available operations.

**Variants:**
- `SetClaudeProfile` - Apply a profile to Claude configuration
- `ResetClaudeProfile` - Remove current Claude profile
- `SetCodexProfile` - Apply a profile to Codex configuration  
- `ResetCodexProfile` - Remove current Codex profile
- `Profile` - Profile management subcommands
- `Completion` - Generate shell completions
- `InternalCompletion` - Hidden completion helpers

#### `enum ProfileCommand`

```rust
#[derive(Debug, Subcommand)]
pub enum ProfileCommand {
    List,
    Edit(ProfileArgs),
    Delete(ProfileArgs),
    Create(ProfileArgs),
    Show(ProfileArgs),
    Copy(ProfileArgs),
}
```

**Location:** [src/cli.rs:61-75](../src/cli.rs#L61-L75)

Profile management subcommands for CRUD operations.

#### `enum Shell`

```rust
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Shell {
    Zsh,
}
```

**Location:** [src/cli.rs:56-59](../src/cli.rs#L56-L59)

Supported shell types for completion generation.

## Storage Module (`storage.rs`)

### Core Storage Types

#### `struct Storage`

```rust
#[derive(Debug, Clone)]
pub struct Storage {
    pub(crate) path: PathBuf,
    pub(crate) config: Config,
}
```

**Location:** [src/storage.rs:5-9](../src/storage.rs#L5-L9)

Main storage interface managing profile data and configuration.

**Fields:**
- `path: PathBuf` - Base storage directory path
- `config: Config` - Loaded configuration data

#### `struct Config`

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Config {
    pub(crate) agents: Agents,
}
```

**Location:** [src/storage.rs:11-14](../src/storage.rs#L11-L14)

TOML configuration structure.

#### `struct Agents`

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct Agents {
    pub(crate) disable_claude: bool,
    pub(crate) disable_codex: bool,
    pub(crate) disable_cline: bool,
}
```

**Location:** [src/storage.rs:16-21](../src/storage.rs#L16-L21)

Agent enable/disable configuration flags.

### Storage Methods

#### `Storage::new(path: PathBuf) -> Result<Self>`

**Location:** [src/storage.rs:50-56](../src/storage.rs#L50-L56)

Create storage instance from explicit path with validation.

**Parameters:**
- `path: PathBuf` - Directory containing config.toml and repo/ subdirectory

**Returns:** `Result<Storage>` - Initialized storage or validation error

**Example:**
```rust
let storage = Storage::new(PathBuf::from("/home/user/.config/pmx"))?;
```

#### `Storage::auto() -> Result<Self>`

**Location:** [src/storage.rs:189-203](../src/storage.rs#L189-L203)

Auto-discover storage location with fallback initialization.

**Returns:** `Result<Storage>` - Storage from discovered/initialized location

**Discovery order:**
1. `$XDG_CONFIG_HOME/pmx` if XDG_CONFIG_HOME set
2. `~/.config/pmx` as fallback
3. Initialize if neither exists

#### `Storage::list_repos() -> Result<Vec<String>>`

**Location:** [src/storage.rs:129-145](../src/storage.rs#L129-L145)

List all available profiles in storage.

**Returns:** `Result<Vec<String>>` - Profile names (without .md extension)

#### `Storage::get_repo_path(path: &str) -> Result<PathBuf>`

**Location:** [src/storage.rs:147-151](../src/storage.rs#L147-L151)

Get filesystem path for a profile by name.

**Parameters:**
- `path: &str` - Profile name

**Returns:** `Result<PathBuf>` - Full path to profile file

#### `Storage::profile_exists(name: &str) -> bool`

**Location:** [src/storage.rs:153-156](../src/storage.rs#L153-L156)

Check if a profile exists without error handling.

#### `Storage::create_profile(name: &str, content: &str) -> Result<()>`

**Location:** [src/storage.rs:158-171](../src/storage.rs#L158-L171)

Create a new profile with given content.

**Parameters:**
- `name: &str` - Profile name (validated)
- `content: &str` - Profile content

#### `Storage::delete_profile(name: &str) -> Result<()>`

**Location:** [src/storage.rs:173-180](../src/storage.rs#L173-L180)

Delete an existing profile.

#### `Storage::get_profile_content(name: &str) -> Result<String>`

**Location:** [src/storage.rs:182-187](../src/storage.rs#L182-L187)

Read profile content from filesystem.

## Commands Module (`commands/`)

### Claude Code Commands

#### `set_claude_profile(storage: &Storage, profile: &str) -> Result<()>`

**Location:** [src/commands/claude_code.rs:3-36](../src/commands/claude_code.rs#L3-L36)

Apply a profile to Claude configuration at `~/.claude/CLAUDE.md`.

**Parameters:**
- `storage: &Storage` - Storage instance
- `profile: &str` - Profile name to apply

**Behavior:**
1. Check if Claude is enabled in configuration
2. Validate profile exists in storage
3. Create `~/.claude/` directory if needed
4. Copy profile to `~/.claude/CLAUDE.md`

#### `reset_claude_profile(storage: &Storage) -> Result<()>`

**Location:** [src/commands/claude_code.rs:38-66](../src/commands/claude_code.rs#L38-L66)

Remove current Claude profile configuration.

**Parameters:**
- `storage: &Storage` - Storage instance (for config validation)

**Behavior:**
1. Check if Claude is enabled in configuration
2. Remove `~/.claude/CLAUDE.md` if it exists
3. Provide feedback about operation result

### Codex Commands

#### `set_codex_profile(storage: &Storage, profile: &str) -> Result<()>`

**Location:** [src/commands/openai_codex.rs:3-36](../src/commands/openai_codex.rs#L3-L36)

Apply a profile to Codex configuration at `~/.codex/AGENTS.md`.

**Parameters:**
- `storage: &Storage` - Storage instance
- `profile: &str` - Profile name to apply

**Behavior:**
1. Check if Codex is enabled in configuration
2. Validate profile exists in storage
3. Create `~/.codex/` directory if needed
4. Copy profile to `~/.codex/AGENTS.md`

#### `reset_codex_profile(storage: &Storage) -> Result<()>`

**Location:** [src/commands/openai_codex.rs:38-66](../src/commands/openai_codex.rs#L38-L66)

Remove current Codex profile configuration.

### Profile Commands

#### `edit(storage: &Storage, name: &str) -> Result<()>`

**Location:** [src/commands/profile.rs:7-26](../src/commands/profile.rs#L7-L26)

Edit an existing profile using the configured editor.

**Parameters:**
- `storage: &Storage` - Storage instance
- `name: &str` - Profile name to edit

**Editor Selection:**
1. `$EDITOR` environment variable
2. `$VISUAL` environment variable fallback
3. Platform-specific defaults (vi/nano/emacs on Unix, notepad on Windows)

#### `delete(storage: &Storage, name: &str) -> Result<()>`

**Location:** [src/commands/profile.rs:28-56](../src/commands/profile.rs#L28-L56)

Delete a profile with user confirmation.

**Behavior:**
1. Display profile content for review
2. Prompt for confirmation (default: no)
3. Delete if confirmed

#### `create(storage: &Storage, name: &str) -> Result<()>`

**Location:** [src/commands/profile.rs:58-121](../src/commands/profile.rs#L58-L121)

Create a new profile using an editor.

**Behavior:**
1. Validate profile name (see `validate_profile_name`)
2. Check profile doesn't already exist
3. Create temporary file with template
4. Open in editor
5. Validate content is not empty/default
6. Save to storage

#### `show(storage: &Storage, name: &str) -> Result<()>`

**Location:** [src/commands/profile.rs:123-127](../src/commands/profile.rs#L123-L127)

Display profile content to stdout.

#### `copy(storage: &Storage, name: &str) -> Result<()>`

**Location:** [src/commands/profile.rs:129-132](../src/commands/profile.rs#L129-L132)

Copy profile content to clipboard.

#### `validate_profile_name(name: &str) -> Result<()>`

**Location:** [src/commands/profile.rs:175-217](../src/commands/profile.rs#L175-L217)

Validate profile name for security and format requirements.

**Validation rules:**
- Non-empty, max 255 characters
- No path traversal (`..`, `\`)
- No invalid filesystem characters (`<>:"|?*`, control chars)
- No empty path components when using `/`
- No `.` or `..` path components

### Utility Commands

#### `list(storage: &Storage) -> Result<()>`

**Location:** [src/commands/utils.rs:1-68](../src/commands/utils.rs#L1-L68)

List profiles with tree structure for terminal output, plain list for pipes.

**Behavior:**
- **Terminal output:** Tree-structured with directories and files
- **Piped output:** Simple newline-separated list

**Example tree output:**
```
├── development/
│   ├── rust
│   └── python
└── general
```

#### `copy_profile(path: &str, storage: &Storage) -> Result<()>`

**Location:** [src/commands/utils.rs:70-82](../src/commands/utils.rs#L70-L82)

Copy profile content to system clipboard using `arboard`.

#### `completion(shell: &Shell) -> Result<()>`

**Location:** [src/commands/utils.rs:84-92](../src/commands/utils.rs#L84-L92)

Generate shell completion scripts.

**Supported shells:**
- Zsh: Loads from embedded completion file

#### `internal_completion(storage: &Storage, completion_cmd: &InternalCompletionCommand) -> Result<()>`

**Location:** [src/commands/utils.rs:94-140](../src/commands/utils.rs#L94-L140)

Provide dynamic completion data for shell completions.

**Completion types:**
- `ClaudeProfiles` - List profiles if Claude enabled
- `CodexProfiles` - List profiles if Codex enabled  
- `EnabledCommands` - List available commands based on config
- `ProfileNames` - List all profile names

## Utils Module (`utils.rs`)

#### `home_dir() -> Result<PathBuf>`

**Location:** [src/utils.rs:1-11](../src/utils.rs#L1-L11)

Get user home directory path with platform-specific handling.

**Platform behavior:**
- **Unix/Linux/macOS:** Uses `std::env::home_dir()` (deprecated but functional)
- **Windows:** Returns error with instructions to set environment variable manually

**Returns:** `Result<PathBuf>` - User home directory path

**Example:**
```rust
let home = pmx::utils::home_dir()?;
let claude_dir = home.join(".claude");
```

## Error Handling Patterns

All public functions use `anyhow::Result<T>` for error handling with context:

```rust
// Adding context to errors
std::fs::copy(&source_file, &system_prompt_location)
    .map_err(|e| anyhow::anyhow!("Failed to apply profile '{}': {}", profile, e))?;

// Using ensure! macro for validation
ensure!(
    !storage.config.agents.disable_claude,
    "Claude profiles are disabled in the configuration."
);

// Context with with_context
Command::new(&editor)
    .arg(&profile_path)  
    .status()
    .with_context(|| format!("Failed to execute editor: {}", editor))?;
```