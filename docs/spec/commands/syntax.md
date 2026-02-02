# Command Syntax

Commands are entered via the ex command line and compiled to typed intents.

## Entry

- `:` enters ex command line from Normal mode
- Command line shows at bottom of screen

## General Format

| Part | Description | Example |
|------|-------------|---------|
| Range | Optional line range | `1,10`, `.,$`, `%` |
| Command | Command name | `w`, `s`, `g` |
| Arguments | Command-specific | filename, pattern, flags |

## Common Patterns

| Pattern | Meaning |
|---------|---------|
| `:w` | Write current buffer |
| `:w filename` | Write to specific file |
| `:%s/old/new/g` | Global substitute |
| `:10,20d` | Delete lines 10-20 |
| `:g/pattern/d` | Delete matching lines |

## Requirements

1. Parsing is deterministic and pure
2. Errors displayed without blocking input
3. Command execution delegates IO to services
4. Core only processes parsed command intent

## Related

- Ranges: [ranges/README.md](ranges/README.md)
- Substitute: [substitute/README.md](substitute/README.md)
- Command line editing: [cmdline/README.md](cmdline/README.md)
