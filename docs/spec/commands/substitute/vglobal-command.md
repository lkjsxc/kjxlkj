# Vglobal Command

Back: [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md)

Execute a command on lines that do NOT match a pattern.

## Syntax

`:v[global]/{pattern}/{command}`

This is the inverse of `:g` — it runs `{command}` on every line that does NOT match `{pattern}`.

## Equivalence

`:v/{pattern}/{cmd}` is equivalent to `:g!/{pattern}/{cmd}`.

## Examples

| Command | Effect |
|---|---|
| `:v/error/d` | Delete all lines that do NOT contain "error" |
| `:v/^$/d` | Delete all non-empty lines (keep only blank lines) |
| `:v/TODO/normal! I// ` | Comment out all lines without "TODO" |
| `:v/\S/d` | Delete all blank lines |

## Range

`:v` accepts a range prefix:

| Command | Effect |
|---|---|
| `:%v/pattern/d` | Apply to entire buffer |
| `:10,20v/pattern/d` | Apply to lines 10-20 |
| `:'<,'>v/pattern/d` | Apply to visual selection |

## Behavior

1. Scan all lines in the range for `{pattern}`.
2. Mark lines that do NOT match.
3. Execute `{command}` on each marked line from top to bottom.

Lines are marked before execution begins, so deletions do not affect which lines are processed.

## Related

- Global command: [/docs/spec/commands/substitute/global-command.md](/docs/spec/commands/substitute/global-command.md)
- Substitute: [/docs/spec/commands/substitute/substitute-command.md](/docs/spec/commands/substitute/substitute-command.md)
