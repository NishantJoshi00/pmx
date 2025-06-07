# PMX

A simple CLI tool to manage and switch between AI agent profiles across different platforms.

## What is PMX?

PMX helps you organize and quickly switch between different AI agent configurations. Instead of manually editing configuration files or copying profile text, PMX lets you store multiple profiles and apply them with a single command.

Think of it like switching between different "personas" or instruction sets for your AI agents - whether you're doing code reviews, writing documentation, or working on specific projects.

## Why Use PMX?

- **Quick Profile Switching**: Change your AI agent's behavior instantly
- **Multi-Platform**: Works with Claude Code, OpenAI Codex, and more
- **Profile Library**: Store and organize multiple profiles in one place
- **Easy Sharing**: Copy profiles to clipboard for quick sharing
- **Zero Setup**: Auto-discovers configuration directories

## 🚀 Installation

```bash
cargo install --path .
```

Or install to a specific location:
```bash
cargo install --path . --root ~/.local
```

## 📋 How to Use

### Basic Workflow

1. **Store your profiles** as `.md` files in `~/.config/pmx/repo/`
2. **List available profiles** to see what you have
3. **Apply a profile** to your AI agent
4. **Reset when needed** to clear the current profile

### Commands

**See what profiles you have:**
```bash
pmx list
```

**Apply a profile to Claude Code:**
```bash
pmx set-claude-profile my-code-reviewer
```

**Apply a profile to OpenAI Codex:**
```bash
pmx set-codex-profile my-documentation-writer
```

**Copy a profile to your clipboard:**
```bash
pmx profile copy project-specific-instructions
```

**Remove the current profile:**
```bash
pmx reset-claude-profile
pmx reset-codex-profile
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
pmx set-claude-profile my-startup-context
# Now Claude knows your company's coding standards and domain
```

## 📁 Profile Organization

PMX stores profiles in `~/.config/pmx/repo/` as Markdown files:

```
~/.config/pmx/
├── config.toml              # Settings
└── repo/                    # Your profiles
    ├── code-reviewer.md     # Focuses on code quality
    ├── tech-writer.md       # Great at documentation
    ├── debugging-expert.md  # Helps solve complex bugs
    └── startup-context.md   # Knows your project specifics
```

Each profile is just a `.md` file containing the instructions you want your AI agent to follow.

## ⚙️ Setup

PMX works out of the box! It automatically:
- Creates the config directory at `~/.config/pmx/`
- Sets up the profile repository in `repo/`
- Configures agent settings in `config.toml`

You can customize the location:
```bash
export PMX_CONFIG_FILE=/path/to/your/config
```

## 🔧 Shell Completions

Make typing commands faster with auto-completion:

```bash
# For Zsh
source <(pmx completion zsh)
```

## 🏗️ How It Works

PMX is built in Rust with a modular architecture:

- **Storage System**: Auto-discovers config directories and manages profiles
- **CLI Interface**: Clean command parsing with clap
- **Agent Modules**: Separate handlers for Claude Code (`~/.claude/CLAUDE.md`) and Codex (`~/.codex/AGENTS.md`)
- **Profile Management**: Simple file-based storage with clipboard integration

The tool follows a configuration-first approach where agent support can be conditionally enabled/disabled via `config.toml`.

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
