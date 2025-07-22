# User Guide

This guide provides comprehensive information on using PMX for managing AI agent profile configurations.

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Basic Commands](#basic-commands)
- [Profile Management](#profile-management)
- [Agent Integration](#agent-integration)
- [Shell Completions](#shell-completions)
- [Advanced Usage](#advanced-usage)
- [Troubleshooting](#troubleshooting)

## Installation

### From Repository

```bash
# Clone the repository
git clone <repository-url>
cd pmx

# Install to ~/.local/bin
cargo install --path . --root ~/.local

# Or install globally
cargo install --path .
```

### Building from Source

```bash
# Build the project
cargo build --release

# The binary will be available at target/release/pmx
./target/release/pmx --help
```

## Configuration

PMX uses automatic configuration discovery with the following precedence:

1. **Command line:** `--config /path/to/config`
2. **Environment:** `PMX_CONFIG_FILE=/path/to/config`
3. **XDG Config:** `$XDG_CONFIG_HOME/pmx/`
4. **Default:** `~/.config/pmx/`

### Directory Structure

PMX expects the following directory structure:

```
~/.config/pmx/
├── config.toml           # Agent configuration
└── repo/                 # Profile storage
    ├── profile1.md
    ├── category/
    │   └── profile2.md
    └── ...
```

### Configuration File

The `config.toml` file controls which agent integrations are enabled:

```toml
[agents]
disable_claude = false
disable_codex = false
```

**Agent configuration options:**
- `disable_claude = true` - Hides Claude-related commands
- `disable_codex = true` - Hides Codex-related commands

## Basic Commands

### Getting Help

```bash
# Show general help
pmx --help

# Show help for specific commands
pmx profile --help
pmx profile create --help
```

### Version Information

```bash
pmx --version
```

### Listing Profiles

```bash
# List all profiles with tree structure
pmx profile list

# Plain list (useful for scripting)
pmx profile list | grep pattern
```

**Example output:**
```
├── development/
│   ├── rust-expert
│   └── python-data
├── design/
│   └── ui-ux-specialist
└── general-assistant
```

## Profile Management

### Creating Profiles

```bash
# Create a new profile
pmx profile create my-profile
```

This command:
1. Validates the profile name
2. Checks if profile already exists
3. Opens your editor with a template
4. Saves the profile if content is added

**Profile naming rules:**
- No empty names or names over 255 characters
- No path traversal characters (`..`, `\`)
- No invalid filesystem characters (`<>:"|?*`)
- Forward slashes allowed for categorization (`category/profile`)

**Editor selection priority:**
1. `$EDITOR` environment variable
2. `$VISUAL` environment variable
3. Platform defaults (vi/nano/emacs on Unix, notepad on Windows)

### Viewing Profiles

```bash
# Display profile content
pmx profile show my-profile

# Show categorized profile
pmx profile show development/rust-expert
```

### Editing Profiles

```bash
# Edit existing profile
pmx profile edit my-profile
```

Opens the profile in your configured editor for modification.

### Copying Profiles

```bash
# Copy profile content to clipboard
pmx profile copy my-profile
```

Uses system clipboard integration via `arboard`.

### Deleting Profiles

```bash
# Delete profile with confirmation
pmx profile delete my-profile
```

This command:
1. Shows the profile content for review
2. Prompts for confirmation (default: no)
3. Deletes if confirmed

## Agent Integration

### Claude Integration

PMX integrates with Claude Code by managing the `~/.claude/CLAUDE.md` file.

#### Apply Claude Profile

```bash
# Apply a profile to Claude
pmx set-claude-profile my-profile

# Apply categorized profile
pmx set-claude-profile development/rust-expert
```

**What this does:**
1. Validates Claude is enabled in configuration
2. Ensures the profile exists
3. Creates `~/.claude/` directory if needed
4. Copies profile content to `~/.claude/CLAUDE.md`

#### Reset Claude Profile

```bash
# Remove current Claude profile
pmx reset-claude-profile
```

This removes the `~/.claude/CLAUDE.md` file, returning Claude to default behavior.

### Codex Integration

PMX integrates with OpenAI Codex by managing the `~/.codex/AGENTS.md` file.

#### Apply Codex Profile

```bash
# Apply a profile to Codex
pmx set-codex-profile my-profile
```

#### Reset Codex Profile

```bash
# Remove current Codex profile
pmx reset-codex-profile
```

## Shell Completions

PMX provides intelligent shell completions for commands and profile names.

### Zsh Completions

#### Installation

```bash
# Generate and install completions
pmx completion zsh > ~/.zsh/completions/_pmx

# Add to your .zshrc if not already present
fpath=(~/.zsh/completions $fpath)
autoload -U compinit && compinit
```

#### Usage

The completions provide:
- Command completion for all available commands
- Profile name completion for profile operations
- Dynamic completion based on your configuration

**Examples:**
```bash
pmx <TAB>                    # Shows all available commands
pmx profile <TAB>            # Shows profile subcommands
pmx profile edit <TAB>       # Shows available profile names
pmx set-claude-profile <TAB> # Shows profiles (if Claude enabled)
```

### Completion Features

- **Context-aware:** Only shows relevant options based on configuration
- **Dynamic:** Profile completions reflect your actual profiles
- **Agent-aware:** Hides disabled agent commands

## Advanced Usage

### Custom Configuration Location

```bash
# Use custom config directory
pmx --config /path/to/custom/pmx profile list

# Set via environment variable
export PMX_CONFIG_FILE=/path/to/custom/pmx
pmx profile list
```

### Profile Organization

Organize profiles using directory structure:

```bash
# Create categorized profiles
pmx profile create development/rust-backend
pmx profile create development/frontend-react
pmx profile create design/ui-components
pmx profile create writing/technical-docs
```

### Scripting with PMX

PMX provides scriptable output for automation:

```bash
# Get plain profile list for scripts
profiles=$(pmx profile list)

# Check if profile exists
if pmx profile show "$profile_name" >/dev/null 2>&1; then
    echo "Profile exists"
fi

# Copy profile content programmatically
content=$(pmx profile show "$profile_name")
```

### Bulk Operations

```bash
# List all profiles for processing
pmx profile list | while read profile; do
    echo "Processing: $profile"
    # Your processing logic here
done

# Backup all profiles
mkdir -p backup
pmx profile list | while read profile; do
    pmx profile show "$profile" > "backup/${profile}.md"
done
```

## Troubleshooting

### Common Issues

#### Storage Directory Not Found

**Error:** `Storage path does not exist`

**Solution:**
```bash
# Initialize storage manually
mkdir -p ~/.config/pmx/repo
echo '[agents]
disable_claude = false
disable_codex = false' > ~/.config/pmx/config.toml
```

#### Profile Not Found

**Error:** `Profile 'name' not found`

**Solution:**
```bash
# List available profiles
pmx profile list

# Check profile name spelling and path
pmx profile show exact-profile-name
```

#### Editor Not Found

**Error:** `No editor found. Please set the EDITOR environment variable.`

**Solution:**
```bash
# Set editor environment variable
export EDITOR=vim  # or nano, emacs, code, etc.

# Or set in your shell profile
echo 'export EDITOR=vim' >> ~/.bashrc
```

#### Agent Commands Not Available

**Issue:** Claude/Codex commands not showing in help

**Solution:**
Check your configuration file:
```bash
# View current config
cat ~/.config/pmx/config.toml

# Enable agents if needed
# Edit the config.toml to set disable_claude = false
```

#### Permission Errors

**Error:** `Failed to create .claude directory: Permission denied`

**Solution:**
```bash
# Check home directory permissions
ls -la ~/

# Ensure you have write access to home directory
# May need to fix ownership or permissions
```

### Getting Debug Information

```bash
# Show current configuration location
pmx --config /dev/null profile list 2>&1 | head -5

# Test storage validation
pmx profile list

# Verify agent configuration
cat ~/.config/pmx/config.toml
```

### Common Workflow Issues

#### Profile Content Not Saved

**Issue:** Editor closes but profile not created

**Causes:**
- Empty content or only template content
- Editor exited with non-zero status
- File system permissions

**Solution:**
1. Ensure you add meaningful content beyond the template
2. Save the file before exiting the editor
3. Check that the editor exits successfully

#### Clipboard Not Working

**Issue:** `pmx profile copy` fails

**Causes:**
- No clipboard manager on Linux
- Wayland compositor clipboard issues
- SSH session without X11 forwarding

**Solutions:**
```bash
# On Linux, ensure clipboard manager is running
# For X11:
xclip -version || sudo apt install xclip

# For Wayland:
wl-copy --version || sudo apt install wl-clipboard

# Alternative: redirect to file
pmx profile show profile-name > profile.md
```

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `PMX_CONFIG_FILE` | Override config directory | `/custom/path/pmx` |
| `XDG_CONFIG_HOME` | XDG base config directory | `/home/user/.config` |
| `EDITOR` | Preferred text editor | `vim`, `nano`, `code` |
| `VISUAL` | Fallback text editor | `emacs` |

## File Locations

| Purpose | Default Location | Description |
|---------|------------------|-------------|
| Configuration | `~/.config/pmx/config.toml` | Agent settings |
| Profile Storage | `~/.config/pmx/repo/` | Profile `.md` files |
| Claude Profile | `~/.claude/CLAUDE.md` | Active Claude configuration |
| Codex Profile | `~/.codex/AGENTS.md` | Active Codex configuration |