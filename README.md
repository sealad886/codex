# OpenAI Codex CLI

**A professional-grade local coding agent that runs on your machine**

<p align="center">
  <code>npm install -g @openai/codex</code><br />
  <em>or</em><br />
  <code>brew install codex</code><br />
  <em>or</em><br />
  <code>make setup && make build</code>
</p>

<p align="center">
  <strong>Codex CLI</strong> is a powerful coding agent from OpenAI that runs locally on your computer, providing AI-assisted development with enterprise-grade security and performance.
</p>

<p align="center">
  <img src="./.github/codex-cli-splash.png" alt="Codex CLI splash" width="80%" />
</p>

---

## 🚀 Quickstart

### One-Command Setup

**Prerequisites**: Rust 1.90+ (or use pre-built binaries)

```bash
# Clone and build from source
git clone https://github.com/openai/codex.git
cd codex
make setup build

# Or install pre-built binary
npm install -g @openai/codex
# OR
brew install codex
```

### First Run

```bash
codex
```

The CLI will guide you through authentication and configuration on first launch.

## 🎯 Core Features

### Interactive Development
- **Full-screen TUI** with real-time conversation
- **File picker** with fuzzy search (`@filename`)
- **Session management** with resume capability
- **Visual diff** display for code changes

### Non-Interactive Automation  
```bash
codex exec "optimize this React component for performance"
codex exec --full-auto "add error handling to all API calls"
codex apply  # Apply agent-generated patches
```

### Enterprise Security
- **Sandboxed execution** with multiple security policies
- **Command allowlisting** and approval workflows  
- **Read-only** (default) and **workspace-write** modes
- **Linux Landlock** and **macOS Seatbelt** integration

### Extensibility
- **Model Context Protocol (MCP)** client and server
- **Custom tool integration** via MCP
- **Configuration profiles** for different workflows

## 📋 Usage Examples

### Interactive Mode (Default)
```bash
# Start interactive session
codex

# Resume last session  
codex resume --last

# Resume specific session
codex resume 1234-5678-9abc-def0
```

### Non-Interactive Mode
```bash
# Execute single task
codex exec "refactor this function to be more readable"

# Full automation mode (minimal prompts)
codex exec --full-auto "add comprehensive tests for user authentication"

# Use specific model and config
codex exec -m gpt-5 -p production "deploy application with zero downtime"
```

### Advanced Operations
```bash
# Apply latest generated diff
codex apply

# Run as MCP server for integration
codex mcp

# Generate shell completions
codex completion bash > ~/.bash_completion.d/codex
```

## ⚙️ Configuration

### Quick Configuration
```bash
# Set default model
codex -c model=gpt-5

# Configure sandbox policy
codex -c sandbox_mode=workspace-write

# Set up profiles
codex -p development  # Use 'development' profile
```

### Configuration File (`~/.codex/config.toml`)
```toml
# Model settings
model = "gpt-5-codex"
model_provider = "openai"

# Security settings
sandbox_mode = "read-only"
approval_policy = "untrusted"

# Performance settings
max_concurrent_requests = 4
request_timeout = 30

# MCP server connections
[mcp_servers.filesystem]
command = "npx"
args = ["@modelcontextprotocol/server-filesystem", "/path/to/workspace"]

# Custom profiles
[profiles.development]
model = "gpt-5"
sandbox_mode = "workspace-write"
approval_policy = "on-failure"

[profiles.production]
model = "o3"
sandbox_mode = "read-only" 
approval_policy = "always"
```

### Environment Variables
Copy `.env.example` to `.env` and customize:
```bash
# Authentication
OPENAI_API_KEY=your-api-key

# Default settings
CODEX_MODEL=gpt-5-codex
CODEX_SANDBOX_MODE=read-only
RUST_LOG=info
```

## 🛡️ Security & Sandboxing

Codex CLI provides multiple layers of security to ensure safe code execution:

### Sandbox Modes
```bash
# Read-only mode (default) - No file modifications
codex --sandbox read-only

# Workspace write - Allow file changes in current directory
codex --sandbox workspace-write  

# Full access - Disable sandboxing (use with caution)
codex --sandbox danger-full-access
```

### Approval Policies
```bash
# Require approval for untrusted commands (default)
codex --ask-for-approval untrusted

# Auto-approve, ask only on failures
codex --ask-for-approval on-failure

# Let model decide when to ask
codex --ask-for-approval on-request

# Never ask (full automation)
codex --ask-for-approval never
```

### Command Safety
The CLI automatically analyzes commands for potential risks:
- ✅ **Safe**: `ls`, `cat`, `grep`, `find`, basic git operations
- ⚠️ **Requires approval**: `rm`, `mv`, `chmod`, network operations  
- 🚫 **Blocked**: `sudo rm -rf`, destructive system operations

## 🏗️ Development & Building

### Prerequisites
- **Rust 1.90+** (install via [rustup](https://rustup.rs/))
- **System dependencies**: OpenSSL, pkg-config

### Build from Source
```bash
# Setup development environment
make setup

# Build debug version
make dev-build

# Build optimized release
make build

# Run tests
make test

# Format and lint
make format lint
```

### Project Structure
```
codex-rs/                  # Rust workspace root
├── cli/                   # Main CLI application
├── core/                  # Core business logic
├── exec/                  # Non-interactive execution
├── tui/                   # Terminal user interface
├── apply-patch/           # Diff application utilities
├── mcp-client/            # MCP client implementation
├── mcp-server/            # MCP server implementation  
└── protocol/              # Shared protocol definitions
```

## 🐳 Docker Support

### Build Container
```bash
make docker-build
```

### Run in Container
```bash
# Interactive mode
docker run -it --rm -v $(pwd):/workspace codex-cli:latest

# Execute command
docker run --rm -v $(pwd):/workspace codex-cli:latest exec "analyze this code"
```

## 🔧 Troubleshooting

### Common Issues

**Authentication Failures**
```bash
# Check login status
codex login status

# Re-authenticate with ChatGPT
codex login

# Use API key instead
codex login --api-key your-key-here
```

**Permission Denied Errors**
```bash
# Enable workspace write permissions
codex --sandbox workspace-write

# Or bypass sandbox for trusted environments
codex --dangerously-bypass-approvals-and-sandbox
```

**Network/Proxy Issues**
```bash
# Set proxy environment variables
export HTTPS_PROXY=http://proxy:8080
export NO_PROXY=localhost,127.0.0.1

# Or configure in ~/.codex/config.toml
[network]
proxy = "http://proxy:8080"
timeout = 60
```

### Debug Mode
```bash
# Enable verbose logging
RUST_LOG=debug codex exec "your command"

# Trace all operations
RUST_LOG=trace codex --help
```

## 🚀 Advanced Features

### Model Context Protocol (MCP)
```bash
# Connect to MCP servers (configured in config.toml)
codex  # Automatically connects to configured servers

# Run Codex as an MCP server
codex mcp

# Test MCP integration
npx @modelcontextprotocol/inspector codex mcp
```

### Session Management  
```bash
# List previous sessions
codex resume  # Shows picker

# Resume last session
codex resume --last

# Resume by ID
codex resume abc-123-def-456

# Sessions are stored in ~/.codex/conversations/
```

### Desktop Notifications
Configure in `~/.codex/config.toml`:
```toml
[notifications]
on_completion = "notify-send 'Codex' 'Task completed'"
on_error = "notify-send 'Codex Error' 'Task failed'"
```

## 📚 API & Integration

### Programmatic Usage
```bash
# JSON output for parsing
codex exec --json "generate API documentation"

# Pipe input
echo "Optimize this SQL query" | codex exec

# Exit codes for scripting
codex exec "run tests" && echo "Tests passed" || echo "Tests failed"
```

### CI/CD Integration
```yaml
# GitHub Actions example
- name: AI Code Review
  run: |
    codex exec --json "review this pull request for security issues" \
      > review-results.json
    
- name: Generate Documentation  
  run: codex exec "update API docs based on code changes"
```

## 📄 License & Contributing

This project is licensed under the [Apache-2.0 License](LICENSE).

### Contributing Guidelines
1. Fork the repository
2. Create a feature branch
3. Run `make ci-test` to ensure all checks pass
4. Submit a pull request

### Development Workflow
```bash
# Setup development environment
make setup

# Run tests in watch mode
cargo watch -x test

# Format code automatically  
make format

# Run full CI suite
make ci-test
```

---

## Support & Resources

- **Documentation**: [docs/](./docs/)
- **FAQ**: [docs/faq.md](./docs/faq.md)  
- **Issues**: [GitHub Issues](https://github.com/openai/codex/issues)
- **Discussions**: [GitHub Discussions](https://github.com/openai/codex/discussions)

### Related Projects
- **IDE Extensions**: [VS Code](https://developers.openai.com/codex/ide), [Cursor](https://cursor.sh), [Windsurf](https://windsurf.ai)
- **Web Interface**: [ChatGPT Codex](https://chatgpt.com/codex)
- **MCP Ecosystem**: [Model Context Protocol](https://modelcontextprotocol.io)

---

<p align="center">
  <strong>Built with ❤️ by OpenAI</strong><br />
  Empowering developers with AI-assisted coding
</p>