# PMX Documentation

PMX is a Rust CLI tool for managing Claude and Codex AI agent profile configurations. This documentation provides comprehensive information about the project's architecture, API, and usage.

## Table of Contents

- [Architecture Overview](./architecture.md) - High-level system design and component relationships
- [API Reference](./api.md) - Detailed documentation of all modules and functions
- [User Guide](./user-guide.md) - Complete usage examples and workflows
- [Development Guide](./development.md) - Information for contributors and developers

## Quick Links

### Core Components

- **CLI Interface** ([src/cli.rs](../src/cli.rs)) - Command-line argument parsing with clap
- **Storage System** ([src/storage.rs](../src/storage.rs)) - Profile storage and configuration management
- **Commands** ([src/commands/](../src/commands/)) - Modular command implementations

### Key Features

- **Profile Management**: Create, edit, delete, list, and copy AI agent profiles
- **Claude Integration**: Apply profiles to `~/.claude/CLAUDE.md`
- **Codex Integration**: Apply profiles to `~/.codex/AGENTS.md`
- **Auto-discovery**: Automatic configuration discovery and initialization
- **Shell Completions**: Built-in completion support for Zsh

## Getting Started

1. **Installation**: See [User Guide - Installation](./user-guide.md#installation)
2. **Basic Usage**: See [User Guide - Basic Commands](./user-guide.md#basic-commands)
3. **Configuration**: See [User Guide - Configuration](./user-guide.md#configuration)

## Project Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root
├── cli.rs               # Command-line interface definitions
├── storage.rs           # Storage and configuration management
├── utils.rs             # Shared utilities
├── commands.rs          # Commands module declaration
└── commands/
    ├── claude_code.rs   # Claude profile management
    ├── openai_codex.rs  # Codex profile management
    ├── profile.rs       # Profile CRUD operations
    └── utils.rs         # Utility commands (list, copy, completions)
```

## Key Design Principles

1. **Configuration-first**: Auto-discovery ensures the tool works without manual setup
2. **Modular Architecture**: Clear separation of concerns across modules
3. **Safety**: Comprehensive validation and error handling
4. **User Experience**: Intuitive commands with helpful feedback

## Contributing

See [Development Guide](./development.md) for information about:
- Setting up the development environment
- Running tests and linting
- Code style and conventions
- Submitting contributions