# PMX Claude

Claude Code agent runner for PMX - run AI agents on your current directory with both Anthropic API and Vertex AI support.

## Overview

`pmx-claude` is an extension for PMX that provides intelligent code agents powered by Claude. It supports both direct Anthropic API access and Google Cloud Vertex AI, with full compatibility with Claude Code environment variables.

## Installation

### Via PMX Extension System

1. **Enable the extension in PMX config:**
   ```toml
   # ~/.config/pmx/config.toml
   [extensions]
   allowed_subcommands = ["claude"]
   ```

2. **Install pmx-claude:**
   ```bash
   npm install -g pmx-claude
   ```

3. **Use via PMX:**
   ```bash
   pmx claude list-agents
   pmx claude run code-review
   ```

### Standalone Installation

```bash
npm install -g pmx-claude
pmx-claude list-agents
```

## Authentication

### Option 1: Direct Anthropic API
```bash
export ANTHROPIC_API_KEY=your_api_key_here
```

### Option 2: Vertex AI (Claude Code Compatible)
```bash
export CLAUDE_CODE_USE_VERTEX=true
export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
export CLOUD_ML_REGION=us-east5  # optional, defaults to us-east5
```

## Available Agents

| Agent | Description | Use Case |
|-------|-------------|----------|
| `code-review` | Analyzes code quality, style, and potential issues | Code audits, best practices |
| `documentation` | Generates or improves project documentation | README, API docs, comments |
| `test-generation` | Creates test files for existing code | Unit tests, test coverage |

## Usage

### List Available Agents
```bash
pmx claude list-agents
```

### Run an Agent
```bash
# Basic usage
pmx claude run code-review

# With options
pmx claude run code-review --verbose --dry-run

# Force specific provider
pmx claude run documentation --provider vertex
pmx claude run test-generation --provider anthropic
```

### Check Provider Status
```bash
pmx claude status
```

## Command Options

### `pmx claude run <agent-name>`

| Option | Description | Default |
|--------|-------------|---------|
| `-v, --verbose` | Enable verbose output | `false` |
| `--dry-run` | Show what the agent would do without executing | `false` |
| `--provider <provider>` | AI provider: `anthropic`, `vertex`, or `auto` | `auto` |
| `--config <path>` | Path to custom agent configuration file | - |

### Provider Auto-Detection

When using `--provider auto` (default), the system will:

1. **Check for Vertex AI**: If `CLAUDE_CODE_USE_VERTEX` and `ANTHROPIC_VERTEX_PROJECT_ID` are set
2. **Fall back to Anthropic API**: If `ANTHROPIC_API_KEY` is available
3. **Error**: If no valid credentials are found

## Examples

### Code Review
```bash
# Quick code review of current directory
pmx claude run code-review

# Detailed review with verbose output
pmx claude run code-review --verbose

# See what would be analyzed without running
pmx claude run code-review --dry-run
```

### Documentation Generation
```bash
# Generate documentation for the project
pmx claude run documentation

# Force using Vertex AI
pmx claude run documentation --provider vertex
```

### Test Generation
```bash
# Generate tests for current codebase
pmx claude run test-generation --verbose
```

## Environment Variables

### Claude Code Compatible
- `CLAUDE_CODE_USE_VERTEX`: Set to enable Vertex AI
- `ANTHROPIC_VERTEX_PROJECT_ID`: GCP project ID for Vertex AI
- `CLOUD_ML_REGION`: GCP region (optional, defaults to us-east5)

### Anthropic API
- `ANTHROPIC_API_KEY`: Your Anthropic API key

## File Patterns

Each agent analyzes different file types:

### Code Review Agent
- `**/*.{js,ts,jsx,tsx,py,rs,go,java,cpp,c,h}`

### Documentation Agent
- `**/*.{js,ts,jsx,tsx,py,rs,go,java}`
- `**/README.md`
- `**/package.json`

### Test Generation Agent
- `**/*.{js,ts,jsx,tsx,py,rs,go,java}`

## Limitations

- **File Size**: Files larger than 100KB are skipped
- **File Count**: Limited to 20 files per agent run to prevent token overflow
- **Excluded Directories**: `node_modules`, `target`, `dist`, `.git`, `build`

## Integration with PMX

This extension integrates seamlessly with PMX's existing profile system. You can:

1. Use PMX profiles as system prompts for agents (future feature)
2. Leverage PMX's configuration management
3. Maintain consistency with other PMX extensions

## Development

### Prerequisites
- Node.js 18+
- TypeScript
- NPM

### Setup
```bash
git clone <repository>
cd pmx-claude
npm install
npm run build
npm link  # For global installation
```

### Scripts
```bash
npm run build      # Compile TypeScript
npm run dev        # Run with ts-node
npm run watch      # Watch mode for development
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes and add tests
4. Submit a pull request

## License

ISC License

## Support

For issues and questions:
- GitHub Issues: [Repository Issues]
- PMX Documentation: [PMX Docs]

---

**Note**: This is Phase 1 implementation with basic agent functionality. Future phases will include custom agent creation, interactive modes, and advanced workflow capabilities.