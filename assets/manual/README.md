# Manual Pages

This directory contains the man pages for pmx.

## Installation

To install the man page system-wide:

```bash
# Copy to system man pages directory
sudo cp pmx.1 /usr/share/man/man1/pmx.1

# Update man page database
sudo mandb
```

For local user installation:

```bash
# Create local man directory if it doesn't exist
mkdir -p ~/.local/share/man/man1

# Copy man page
cp pmx.1 ~/.local/share/man/man1/pmx.1

# Add to MANPATH if needed (add to your shell profile)
export MANPATH="$HOME/.local/share/man:$MANPATH"
```

## Viewing

View the man page:

```bash
# From this directory (for testing)
man ./pmx.1

# After installation
man pmx
```

## Generation

The man page is currently maintained manually. Future versions may include automated generation from the CLI help text using clap_mangen.