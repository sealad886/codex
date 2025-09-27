# Codex Ignore Files

Codex supports hierarchical ignore files that control which files are included in file searches and other operations. The ignore system supports `.gitignore` (Git standard), `.aiignore` (alias), and `.codexignore` (canonical) files with gitignore-compatible semantics. 

## File Types

- **`.gitignore`** - The Git standard ignore file (processed first)
- **`.aiignore`** - An alias for compatibility with other tools (processed second)  
- **`.codexignore`** - The canonical ignore file format (processed last, highest precedence)

When multiple files exist in the same directory, they are processed in order with later files able to override earlier ones: `.gitignore` → `.aiignore` → `.codexignore`.

## Precedence Hierarchy

Ignore files are processed in the following order (lowest to highest precedence):

1. **System global files** (lowest precedence)
   - Unix/Linux: `/etc/codex/ignore`, `/etc/ai/ignore`
   - Windows: `%ProgramData%\codex\ignore`, `%ProgramData%\ai\ignore`

2. **User global files**
   - Unix/Linux: `${XDG_CONFIG_HOME:-$HOME/.config}/codex/ignore`, `${XDG_CONFIG_HOME:-$HOME/.config}/ai/ignore`
   - Windows: `%AppData%\codex\ignore`, `%AppData%\ai\ignore`

3. **Project root files**
   - `.gitignore` in the repository root
   - `.aiignore` in the repository root
   - `.codexignore` in the repository root

4. **Directory-level files** (highest precedence)
   - `.gitignore`, `.aiignore` and `.codexignore` in each directory, with files closer to the target path taking precedence

Within the same directory and precedence level:
- `.gitignore` files are processed first
- `.aiignore` files are processed second
- `.codexignore` files are processed third (and can override `.gitignore` and `.aiignore` patterns)

## Pattern Syntax

Ignore files use gitignore-compatible pattern syntax:

### Basic Patterns
```
# Comments start with #
*.tmp           # Ignore all .tmp files
*.log           # Ignore all .log files
build/          # Ignore build directory (trailing slash for directories only)
/dist           # Ignore dist directory at root level (leading slash for root-anchored)
```

### Wildcards
```
*.txt           # Single * matches anything except /
**/*.log        # Double ** matches across directory boundaries
test?.js        # ? matches any single character
[abc].txt       # Character classes match one of the specified characters
```

### Negation
```
*.log           # Ignore all .log files
!debug.log      # But don't ignore debug.log (negation with !)
!important/*.log # Don't ignore .log files in important/ directory
```

### Escaping
```
\#not-comment   # Literal # at start of line (escaped)
\*.txt          # Literal * in filename
```

## Environment Variables

- `GIT_IGNORE_FILE` - Comma-separated list of additional .gitignore files to include
- `AI_IGNORE_FILE` - Comma-separated list of additional .aiignore files to include  
- `CODEX_IGNORE_FILE` - Comma-separated list of additional .codexignore files to include
- `GIT_IGNORE_DISABLE=1` - Disable processing of .gitignore files
- `AI_IGNORE_DISABLE=1` - Disable processing of .aiignore files
- `CODEX_IGNORE_DISABLE=1` - Disable processing of .codexignore files
- `CODEX_NO_IGNORE=1` - Disable all ignore file processing

## CLI Options

The file search tool supports several ignore-related flags:

### Basic Control
- `--no-ignore` - Disable all ignore file processing
- `--ignore-file <path>` - Add additional ignore files (can be repeated)

### Family Control (for debugging)
- `--only-gitignore` - Only process .gitignore files (disable .aiignore and .codexignore)
- `--only-aiignore` - Only process .aiignore files (disable .gitignore and .codexignore)
- `--only-codexignore` - Only process .codexignore files (disable .gitignore and .aiignore)

### Debugging and Inspection
- `--print-effective-ignore` - Show all ignore rules in precedence order and exit
- `--explain-ignore <path>` - Show why a specific path was ignored or included

## Examples

### Basic Usage

Create a `.codexignore` file in your project root:
```
# Build outputs
/target/
/dist/
/build/

# Dependencies
node_modules/
__pycache__/

# Logs
*.log
!debug.log    # Keep debug logs

# Temporary files
*.tmp
*.swp
.DS_Store
```

### Debugging Ignore Rules

Show all effective ignore rules:
```bash
codex-file-search --print-effective-ignore
```

Check why a specific file is ignored:
```bash
codex-file-search --explain-ignore src/temp.log
```

Test with only .aiignore files:
```bash
codex-file-search --only-aiignore pattern
```

### Mixed Usage

You can use `.gitignore`, `.aiignore` and `.codexignore` files together:

`.gitignore`:
```
*.log
*.tmp
```

`.aiignore`:
```
!debug.log    # Override .gitignore to keep debug.log
```

`.codexignore`:
```
!important.log    # Override .gitignore to keep important.log
/private/         # Add additional ignore rule
```

Result: All `.log` files except `debug.log` and `important.log` are ignored, plus `.tmp` files and the `/private/` directory.

## Migration from .gitignore

Codex ignore files use the same syntax as `.gitignore`, so you can:

1. Copy your `.gitignore` patterns to `.codexignore`
2. Add Codex-specific patterns as needed
3. Use negation patterns to fine-tune what should be included in Codex operations vs. git

## Performance Notes

- Ignore file processing is optimized and cached per directory
- Empty ignore files or directories with no ignore files have minimal overhead
- The ignore system is designed to handle large repositories efficiently
- Use `--no-ignore` for maximum performance when ignore processing isn't needed

## Best Practices

1. **Use `.codexignore` as the canonical format** - while `.aiignore` is supported for compatibility, prefer `.codexignore` for new projects

2. **Structure patterns from general to specific** - place broad patterns early and use negation for exceptions

3. **Use directory-specific ignore files** - place ignore files in subdirectories for fine-grained control

4. **Test your patterns** - use `--explain-ignore` to verify that files are being ignored as expected

5. **Document complex patterns** - use comments to explain non-obvious ignore rules

6. **Keep it simple** - avoid overly complex patterns that are hard to understand and maintain