# Command-Line Completion

Custom completion sources and wildmenu behavior.

## Overview

Tab completion in command-line mode expands commands, file paths, buffer names, options, and custom sources.

## Built-in Completion Types (normative)

| Type | Completes | Trigger context |
|---|---|---|
| `command` | Ex commands | After `:` at first word |
| `file` | File paths | After commands that take file arguments |
| `dir` | Directory paths | Same, directory-only |
| `buffer` | Buffer names | After `:b`, `:sb`, etc. |
| `option` | Options | After `:set` |
| `help` | Help topics | After `:help` |
| `color` | Color schemes | After `:colorscheme` |
| `highlight` | Highlight groups | After `:highlight` |
| `filetype` | File types | After `:setfiletype` |
| `shellcmd` | Shell commands | After `:!` |
| `event` | Autocommand events | After `:autocmd` |

## Wildmode (normative)

The `wildmode` option controls completion cycling behavior. It is a comma-separated list applied in order on successive Tab presses.

| Component | Behavior |
|---|---|
| (empty) | Complete to longest common prefix |
| `full` | Complete to next full match, cycling |
| `longest` | Complete to longest common string |
| `longest:full` | Longest first, then cycle full matches |
| `list` | Show all matches in a list |
| `list:full` | List matches, then cycle full |
| `list:longest` | List matches, then complete to longest |

Default: `full`.

## Completion Keys (normative)

| Key | Action |
|---|---|
| `Tab` | Next completion |
| `Shift-Tab` | Previous completion |
| `Ctrl-n` | Next (same as Tab) |
| `Ctrl-p` | Previous (same as Shift-Tab) |
| `Ctrl-y` | Accept current completion |
| `Ctrl-e` | Cancel completion, restore original text |

## Wildmenu Display

When `wildmenu` is enabled (default: true), completions are displayed in a horizontal bar above the command line, with the current selection highlighted.

## File Completion Details

`wildignore` option: comma-separated glob patterns for files to exclude from completion (e.g., `*.o,*.pyc,target/**`).

## Fuzzy Matching

When `wildoptions` contains `fuzzy`, completion uses fuzzy matching: characters need not be contiguous but must appear in order. Scoring prefers prefix matches and consecutive character runs.

## Related

- Command-line entry: [/docs/spec/commands/cmdline/cmdline-entry.md](/docs/spec/commands/cmdline/cmdline-entry.md)
- Command-line editing: [/docs/spec/commands/cmdline/cmdline-editing.md](/docs/spec/commands/cmdline/cmdline-editing.md)
