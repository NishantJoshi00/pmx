<div align="center">

<!-- Light mode logo -->
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/dark.png">
  <source media="(prefers-color-scheme: light)" srcset="assets/light.png">
  <img alt="PMX Logo" src="assets/light.png" width="200">
</picture>

# PMX

A simple CLI tool to manage and switch between AI agent profiles across different platforms.

</div>

## What is PMX?

PMX helps you organize and quickly switch between different AI agent configurations. Instead of manually editing configuration files or copying profile text, PMX lets you store multiple profiles and apply them with a single command.

Think of it like switching between different "personas" or instruction sets for your AI agents - whether you're doing code reviews, writing documentation, or working on specific projects.

## Why Use PMX?

- **Quick Profile Switching**: Change your AI agent's behavior instantly
- **Multi-Platform**: Works with Claude Code, OpenAI Codex, and more
- **Profile Library**: Store and organize multiple profiles in one place
- **Easy Sharing**: Copy profiles to clipboard for quick sharing
- **Zero Setup**: Auto-discovers configuration directories

## ✨ Features

- **Profile Management**: Create, edit, delete, and show profiles with full CRUD operations
- **Editor Integration**: Edit profiles using your preferred `$EDITOR`
- **Append Mode**: Add profiles to existing configurations without overwriting
- **Nested Organization**: Organize profiles in directories for better structure
- **Smart Display**: Tree-style output in terminal, simple list when piped
- **Clipboard Support**: Copy profile contents directly to clipboard
- **Shell Completions**: Tab completion for Zsh (more shells coming soon)
- **Configuration Control**: Enable/disable specific agents via `config.toml`

## 🚀 Installation

### From Source

```bash
cargo install --path .
```

Or install to a specific location:
```bash
cargo install --path . --root ~/.local
```

### Building from Source

```bash
cargo build --release
```

The binary will be available at `target/release/pmx`

## 📋 How to Use

### Basic Workflow

1. **Store your profiles** as `.md` files in `~/.config/pmx/repo/`
2. **List available profiles** to see what you have
3. **Apply a profile** to your AI agent
4. **Reset when needed** to clear the current profile

### Commands

**See what profiles you have:**
```bash
pmx profile list
```

**Apply a profile to Claude Code:**
```bash
pmx set-claude-profile my-code-reviewer
```

**Append to existing Claude profile:**
```bash
pmx append-claude-profile additional-instructions
```

**Apply a profile to OpenAI Codex:**
```bash
pmx set-codex-profile my-documentation-writer
```

**Append to existing Codex profile:**
```bash
pmx append-codex-profile additional-context
```

**Remove the current profile:**
```bash
pmx reset-claude-profile
pmx reset-codex-profile
```

### Profile Management Commands

**Create a new profile:**
```bash
pmx profile create my-new-profile
```

**Edit an existing profile:**
```bash
pmx profile edit my-profile
```

**Show profile contents:**
```bash
pmx profile show my-profile
```

**Copy a profile to your clipboard:**
```bash
pmx profile copy project-specific-instructions
```

**Delete a profile (with confirmation):**
```bash
pmx profile delete old-profile
```

### Example Use Cases

**Code Review Profile:**
```bash
pmx set-claude-profile code-reviewer
# Now Claude will focus on security, performance, and best practices
```

**Documentation Writer:**
```bash
pmx set-claude-profile tech-writer
# Now Claude will write clear, user-friendly documentation
```

**Project-Specific Instructions:**
```bash
pmx set-claude-profile projects/startup
# Now Claude knows your company's coding standards and domain
```

**Development Profiles in Nested Directories:**
```bash
pmx set-claude-profile development/backend
# Apply backend-specific development guidelines

pmx append-claude-profile development/security
# Add security-focused instructions to existing profile
```

## 📁 Profile Organization

PMX stores profiles in `~/.config/pmx/repo/` as Markdown files. You can organize profiles in nested directories:

```
~/.config/pmx/
├── config.toml              # Settings
└── repo/                    # Your profiles
    ├── code-reviewer.md     # Focuses on code quality
    ├── tech-writer.md       # Great at documentation
    ├── development/         # Development profiles
    │   ├── backend.md       # Backend-specific instructions
    │   └── frontend.md      # Frontend guidelines
    └── projects/            # Project-specific profiles
        ├── startup.md       # Startup context
        └── enterprise.md    # Enterprise standards
```

Each profile is just a `.md` file containing the instructions you want your AI agent to follow. Use directories to organize related profiles together.

## ⚙️ Setup

PMX works out of the box! It automatically:
- Creates the config directory at `~/.config/pmx/`
- Sets up the profile repository in `repo/`
- Configures agent settings in `config.toml`

### Custom Configuration Location

You can override the default configuration directory in two ways:

**Using command-line option:**
```bash
pmx --config /path/to/custom/config profile list
```

**Using environment variable:**
```bash
export PMX_CONFIG_FILE=/path/to/your/config
pmx profile list
```

The priority order is:
1. `--config` command-line option
2. `$PMX_CONFIG_FILE` environment variable
3. `$XDG_CONFIG_HOME/pmx` (if XDG_CONFIG_HOME is set)
4. `~/.config/pmx` (default)

## 🔧 Shell Completions

Make typing commands faster with auto-completion:

```bash
# For Zsh
source <(pmx completion zsh)
```

## 📚 Documentation

Man pages are available in `assets/manual/`. To install:

```bash
# System-wide installation
sudo cp assets/manual/pmx.1 /usr/share/man/man1/

# User installation
mkdir -p ~/.local/share/man/man1
cp assets/manual/pmx.1 ~/.local/share/man/man1/
```

Then view with: `man pmx`

## 🏗️ How It Works

PMX is built in Rust with a modular architecture:

- **Storage System**: Auto-discovers config directories and manages profiles
- **CLI Interface**: Clean command parsing with clap
- **Agent Modules**: Separate handlers for Claude Code (`~/.claude/CLAUDE.md`) and Codex (`~/.codex/AGENTS.md`)
- **Profile Management**: Full CRUD operations with editor integration and clipboard support
- **Smart Output**: Tree-style display in terminal, simple list when piped (using `is-terminal`)
- **Append Mode**: Add profiles to existing configurations without overwriting
- **Interactive Features**: Confirmation dialogs for destructive operations (using `dialoguer`)

The tool follows a configuration-first approach where agent support can be conditionally enabled/disabled via `config.toml`.

### Key Dependencies

- `clap` - Command-line argument parsing
- `anyhow` - Error handling
- `serde`/`toml` - Configuration management
- `arboard` - Clipboard integration
- `dialoguer` - Interactive prompts
- `is-terminal` - Terminal detection for smart output
- `tempfile` - Safe temporary file handling

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Test with `cargo test`
4. Format with `cargo fmt`
5. Submit a pull request

---

<div align="center">
Built by Human, Documented by LLM.
</div>
