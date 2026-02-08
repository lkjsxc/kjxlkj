# Global Command

Back: [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md)

Execute a command on all lines matching a pattern.

## Overview

`:global` (`:g`) scans lines for a pattern match and executes a command on each matching line.

## Syntax

`:[range]g[lobal]/{pattern}/{command}`

## Examples

| Command | Effect |
|---|---|
| `:g/error/d` | Delete all lines containing "error" |
| `:g/^$/d` | Delete all blank lines |
| `:g/TODO/p` | Print all lines containing "TODO" |
| `:g/pattern/normal! dd` | Delete matching lines via normal command |
| `:g/^import/m 0` | Move all import lines to top of file |

## Execution order

1. All lines in the range are scanned for `{pattern}`.
2. Matching lines are marked.
3. `{command}` is executed on each marked line, top to bottom.

The two-pass approach means line deletions do not affect which lines are processed.

## Range

Default range is `%` (entire buffer).

| Range | Description |
|---|---|
| `:%g/pattern/cmd` | Entire buffer (default) |
| `:10,20g/pattern/cmd` | Lines 10-20 |
| `:'<,'>g/pattern/cmd` | Visual selection |

## Inverse

`:g!/{pattern}/{cmd}` or `:v/{pattern}/{cmd}` executes `{command}` on non-matching lines.

## Common Patterns

| Command | Purpose |
|---|---|
| `:g/^$/d` | Remove blank lines |
| `:g/pattern/t $` | Copy matching lines to end of file |
| `:g/pattern/m 0` | Move matching lines to top |
| `:g/pattern/s/old/new/g` | Substitute only on matching lines |

## Chaining

`:g` can execute any ex command, including `:substitute`, `:normal`, `:move`, `:copy`, `:delete`.

## Related

- Vglobal: [/docs/spec/commands/substitute/vglobal-command.md](/docs/spec/commands/substitute/vglobal-command.md)
- Substitute: [/docs/spec/commands/substitute/substitute-command.md](/docs/spec/commands/substitute/substitute-command.md)
- Normal command: [/docs/spec/commands/execution/normal-command.md](/docs/spec/commands/execution/normal-command.md)
