# Insert Mode

Back: [/docs/spec/modes/README.md](/docs/spec/modes/README.md)
Insert mode specifications for text entry.

## Entry Keys

| Key | Action |
|-----|--------|
| `i` | Insert before cursor |
| `a` | Append after cursor |
| `I` | Insert at line start |
| `A` | Append at line end |
| `o` | Open line below |
| `O` | Open line above |

## Insert Mode Keys

| Key | Action |
|-----|--------|
| `Esc` | Return to Normal |
| `Ctrl-o` | Execute one Normal command |
| `Ctrl-w` | Delete word before cursor |
| `Ctrl-u` | Delete to line start |
| `Ctrl-r {reg}` | Insert register contents |

## Directory Structure

| Directory | Content |
|-----------|---------|
| [completion/](completion/README.md) | Completion |
| [input/](input/README.md) | Special input |

## Documents

| Document | Content |
|----------|---------|
| [insert.md](insert.md) | Overview |
| [insert-commands.md](insert-commands.md) | Commands |
| [insert-autopairs.md](insert-autopairs.md) | Auto pairs |
| [insert-indentation.md](insert-indentation.md) | Indentation |
| [insert-mappings.md](insert-mappings.md) | Mappings |
| [insert-navigation.md](insert-navigation.md) | Navigation |
| [insert-normal.md](insert-normal.md) | Ctrl-o mode |

## Related

- Modes: [docs/spec/modes/README.md](/docs/spec/modes/README.md)
- Completion: [completion/README.md](completion/README.md)
