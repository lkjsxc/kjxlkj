# Command Syntax

Back: [/docs/spec/commands/README.md](/docs/spec/commands/README.md)

Commands are entered via the ex command line and compiled to typed intents.

## Entry (normative)

| Key | Action |
|---|---|
| `:` | Enter command-line mode from Normal mode |
| `/` | Enter forward search mode |
| `?` | Enter backward search mode |
| `!` | Filter (when preceded by a range) |

The command line is rendered at the bottom of the terminal, replacing the statusline area.

## General format (normative)

A command string has this grammar:

| Part | Required | Description | Examples |
|------|----------|-------------|---------|
| Range | Optional | Line range specifier | `1,10`, `.,$`, `%`, `'<,'>` |
| Command | Required | Command name (abbreviated or full) | `w`, `write`, `s`, `substitute` |
| `!` suffix | Optional | Force flag | `:w!`, `:q!` |
| Arguments | Optional | Command-specific parameters | filename, pattern, flags |

## Command name abbreviation (normative)

Commands can be abbreviated to their shortest unambiguous prefix:

| Full name | Shortest abbreviation |
|---|---|
| `write` | `w` |
| `quit` | `q` |
| `edit` | `e` |
| `substitute` | `s` |
| `global` | `g` |
| `buffer` | `b` |
| `split` | `sp` |
| `vsplit` | `vsp` |
| `bnext` | `bn` |
| `bprevious` | `bp` |
| `delete` | `d` |
| `yank` | `y` |
| `set` | `se` |
| `help` | `h` |

## Parsing pipeline (normative)

1. **Tokenize**: Split command string into range, command name, arguments.
2. **Resolve**: Match command name against the command registry (including abbreviations).
3. **Parse arguments**: Each command defines its own argument parser.
4. **Validate**: Check range validity, argument types, permission flags.
5. **Produce intent**: Convert to a typed `CommandIntent` value.
6. **Dispatch**: Send intent to core task for execution.

Parsing is deterministic and pure. No side effects occur during parsing.

## Bar-separated commands (normative)

Multiple commands can be separated by `|`:

| Example | Meaning |
|---|---|
| `:w \| q` | Write then quit |
| `:g/pattern/d \| w` | Delete matching lines then write |

The `|` character inside pattern arguments MUST be escaped or handled by the pattern parser, not the command separator.

## Error handling (normative)

| Error type | Behavior |
|---|---|
| Unknown command | Display "Not an editor command: {cmd}" |
| Wrong argument count | Display command usage |
| Invalid range | Display "Invalid range" |
| File not found | Display "Can't open file: {path}" |
| Unsaved changes | Display "No write since last change" (use `!` to force) |

Errors are displayed in the command-line area without blocking input. The user can dismiss errors by pressing any key.

## Related

- Ranges: [/docs/spec/commands/ranges/ranges.md](/docs/spec/commands/ranges/ranges.md)
- Substitute: [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md)
- Command-line editing: [/docs/spec/commands/cmdline/README.md](/docs/spec/commands/cmdline/README.md)
- Essential commands: [/docs/spec/commands/essential.md](/docs/spec/commands/essential.md)
