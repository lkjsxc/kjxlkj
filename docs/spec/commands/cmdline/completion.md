# Command-Line Completion

Back: [docs/spec/commands/cmdline/README.md](/docs/spec/commands/cmdline/README.md)

Tab-completion for ex command input.

## Overview

Command-line completion provides context-aware
suggestions as the user types ex commands. It
completes command names, file paths, option names,
buffer names, and other arguments.

## Activation

Completion is triggered by keypress or automatically.

### Tab Trigger

Pressing `Tab` in command-line mode triggers
completion for the current argument position.

### Wildmenu Display

Completion candidates appear in a horizontal menu
above the command line (wildmenu). The selected
candidate is highlighted.

## Completion Contexts

Different argument positions complete different types of values.

### Command Names

After `:`, completion suggests valid ex commands.
Matches are filtered by the typed prefix.

| Input | Completions |
|-------|-------------|
| `:e` | `:edit`, `:earlier`, ... |
| `:w` | `:write`, `:wall`, `:wq`, ... |
| `:set` | `:set`, `:setlocal`, ... |

### File Paths

For commands that take file arguments (`:edit`,
`:write`, `:source`), completion shows file names
from the filesystem.

| Feature | Description |
|---------|-------------|
| Directory expansion | Tab into directories |
| Hidden files | Shown only if prefix starts with `.` |
| Glob patterns | `*` expands in file arguments |

### Option Names

After `:set`, completion suggests option names.
After `:set option=`, completion suggests valid
values for the option (if enumerable).

### Buffer Names

For `:buffer`, `:sbuffer`, and similar commands,
completion suggests from open buffer names and
numbers.

### Help Tags

For `:help`, completion searches the help index.

### Color Schemes

For `:colorscheme`, completion lists available
themes.

## Navigation

| Key | Action |
|-----|--------|
| `Tab` | Next candidate |
| `Shift-Tab` | Previous candidate |
| `Ctrl-n` | Next candidate |
| `Ctrl-p` | Previous candidate |
| `Enter` | Accept and execute |
| `Esc` | Cancel completion |
| `Ctrl-e` | Dismiss menu, keep text |

## Wildmenu Options

Settings controlling wildmenu appearance and behavior.

### Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `wildmenu` | bool | true | Enable wildmenu |
| `wildmode` | string | "full" | Completion mode |
| `wildignore` | string | "" | Patterns to exclude |
| `wildignorecase` | bool | false | Case-insensitive |

### Wildmode Values

| Value | Behavior |
|-------|----------|
| "full" | Complete to first match, cycle |
| "longest" | Complete to longest common prefix |
| "longest:full" | Longest first, then full |
| "list" | Show list, then complete |
| "list:full" | List, then cycle |

## Custom Completion

User-defined commands can provide their own completion sources.

### User Commands

User-defined commands can specify custom completion
functions. The function receives the current argument
text and returns a list of candidates.

## Fuzzy Completion

Fallback to fuzzy matching when prefix matching finds no results.

### Behavior

When exact prefix matching yields no results, the
completion system falls back to fuzzy matching.
Fuzzy matching uses the same scoring algorithm as
the finder.

## Acceptance Criteria

- Completion MUST NOT mutate editor state
- Completion MUST be synchronous (no async fetches)
- File completion MUST respect `.gitignore` patterns
- Candidates MUST update on each keystroke

## Related

- Command-line mode: [docs/spec/modes/cmdline/README.md](/docs/spec/modes/command.md)
- Command history: [docs/spec/commands/cmdline/history.md](/docs/spec/commands/cmdline/history.md)
