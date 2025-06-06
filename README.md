# PMX

A CLI tool for managing Claude profile configurations.

## Installation

Install PMX directly from the repository using Cargo:

```bash
cargo install --path . --root ~/.local
```

This will install the `pmx` binary to `~/.local/bin/pmx`. Make sure `~/.local/bin` is in your PATH.

Alternatively, you can install it globally:

```bash
cargo install --path .
```

## Usage

PMX manages Claude profile configurations by storing them in a local repository and applying them to `~/.claude/CLAUDE.md`.

### Commands

- **List available profiles:**
  ```bash
  pmx list
  ```

- **Set a Claude profile:**
  ```bash
  pmx set-claude-profile <profile-name>
  ```

- **Reset the current Claude profile:**
  ```bash
  pmx reset-claude-profile
  ```

- **Generate shell completions:**
  ```bash
  pmx completion zsh
  ```

### Configuration

PMX automatically creates a configuration directory at:
- `$XDG_CONFIG_HOME/pmx` (if XDG_CONFIG_HOME is set)
- `~/.config/pmx` (fallback)

You can also specify a custom config location:
```bash
pmx --config /path/to/config <command>
```

Or set the environment variable:
```bash
export PMX_CONFIG_FILE=/path/to/config
```

### Setting up Shell Completions

#### Zsh

To enable zsh completions, add the following to your `.zshrc`:

```bash
# Create a directory for completions if it doesn't exist
mkdir -p ~/.local/share/zsh/completions

# Generate and save the completion script
pmx completion zsh > ~/.local/share/zsh/completions/_pmx

# Add the completion directory to fpath (add this to your .zshrc)
fpath=(~/.local/share/zsh/completions $fpath)

# Initialize completions
autoload -U compinit && compinit
```

## Configuration Structure

PMX expects the following directory structure:

```
~/.config/pmx/
├── config.toml          # Configuration file
└── repo/                # Profile repository
    ├── profile1.md      # Claude profile files
    ├── profile2.md
    └── ...
```

The `config.toml` file contains agent configuration:

```toml
[agents]
disable_claude = false
disable_codex = false
disable_cline = false
```