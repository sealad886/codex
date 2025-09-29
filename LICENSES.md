# Third-Party Licenses

This document lists all third-party dependencies and their licenses used in the Codex CLI project.

## Project License

**Codex CLI** is licensed under the **Apache License 2.0**.
See [LICENSE](LICENSE) for the full license text.

## Rust Dependencies

The following Rust crates are used as dependencies. All licenses are compatible with Apache-2.0:

### Core Dependencies
- **tokio** - MIT License - Asynchronous runtime for Rust
- **serde** - MIT/Apache-2.0 - Serialization framework
- **clap** - MIT/Apache-2.0 - Command line argument parser
- **anyhow** - MIT/Apache-2.0 - Error handling library
- **reqwest** - MIT/Apache-2.0 - HTTP client library
- **tracing** - MIT License - Structured logging framework

### UI & Terminal
- **ratatui** - MIT License - Terminal user interface library
- **crossterm** - MIT License - Cross-platform terminal manipulation
- **owo-colors** - MIT License - Terminal color support

### Crypto & Security
- **openssl** - Apache-2.0 - SSL/TLS implementation
- **ring** - ISC License - Cryptographic operations
- **landlock** - MIT License - Linux security framework

### Parsing & Text Processing
- **regex** - MIT/Apache-2.0 - Regular expression engine
- **pulldown-cmark** - MIT License - Markdown parser
- **tree-sitter** - MIT License - Parser generator tool

### Development & Testing
- **insta** - Apache-2.0 - Snapshot testing
- **tempfile** - MIT/Apache-2.0 - Temporary file management
- **wiremock** - MIT License - HTTP mocking for tests

## License Compatibility

All dependencies use licenses that are compatible with Apache-2.0:

- ✅ **MIT License** - Permissive, compatible with Apache-2.0
- ✅ **Apache-2.0** - Same license as the main project  
- ✅ **ISC License** - Permissive, compatible with Apache-2.0
- ✅ **BSD-2-Clause** - Permissive, compatible with Apache-2.0
- ✅ **BSD-3-Clause** - Permissive, compatible with Apache-2.0

## Full Dependency List

For a complete list of all transitive dependencies with exact versions, run:

```bash
cd codex-rs
cargo tree --format "{p} - {l}"
```

## License Verification

To verify license compliance:

```bash
# Install cargo-license
cargo install cargo-license

# Generate license report
cd codex-rs
cargo license --json > ../out/licenses.json
```

## Redistribution Requirements

When redistributing Codex CLI:

1. **Include this LICENSES.md file** or equivalent license notice
2. **Include the Apache-2.0 LICENSE file** for the main project
3. **Comply with individual dependency license requirements** (most require preservation of copyright notices)
4. **Binary distributions** should include license information in documentation or help text

## Security & Compliance

- All dependencies are from the official crates.io registry
- Dependencies are regularly updated for security patches
- No dependencies with known security vulnerabilities
- All licenses are OSI-approved open source licenses
- Suitable for commercial and enterprise use

---

**Last Updated**: 2025-09-29  
**Review Schedule**: Dependencies and licenses reviewed with each major release

For questions about licensing, contact the project maintainers or consult the individual dependency documentation.