# Filtering and Piping

Back: [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)

Filter buffer text through external commands and insert external command output.

## Filter operator (normative)

The `!` operator filters a range of lines through an external command. The command's stdout replaces the filtered lines.

| Syntax | Action |
|---|---|
| `!{motion}{cmd}` | Filter lines covered by motion through `{cmd}` |
| `!!{cmd}` | Filter current line through `{cmd}` (linewise) |
| `{count}!!{cmd}` | Filter `{count}` lines through `{cmd}` |
| `:{range}!{cmd}` | Filter `{range}` lines through `{cmd}` |
| `:%!{cmd}` | Filter entire buffer through `{cmd}` |

## Read command output (normative)

| Command | Action |
|---|---|
| `:r !{cmd}` | Insert stdout of `{cmd}` below the current line |
| `:{line}r !{cmd}` | Insert stdout of `{cmd}` below line `{line}` |

## Write to command (normative)

| Command | Action |
|---|---|
| `:w !{cmd}` | Send entire buffer to `{cmd}` stdin (buffer is NOT modified) |
| `:{range}w !{cmd}` | Send `{range}` lines to `{cmd}` stdin |

Note: `:w !cmd` (with space) writes to command stdin. `:w!` (without space) force-writes to file.

## Execution model

| Requirement | Detail |
|---|---|
| Shell | Commands run via the shell specified by `shell` option (default: `$SHELL` or `/bin/sh`) |
| Shell flags | The `shellcmdflag` option (default: `-c`) is passed before the command string |
| Timeout | Commands MUST have a configurable timeout (default: 30 seconds). On timeout, the child process is killed. |
| Error display | If the command exits with non-zero status, stderr output is displayed in the message area |
| Encoding | Filter input and output use the buffer's `fileencoding` |

## Undo behavior

A filter operation (replacement of lines) is a single undo entry. `u` restores the original lines.

## Security

Project-local config MUST NOT override the `shell` option. This is a restricted setting per [/docs/spec/features/session/project-config.md](/docs/spec/features/session/project-config.md).

## Related

- Text manipulation: [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)
- Sorting/alignment: [/docs/spec/editing/text-manipulation/sorting-alignment.md](/docs/spec/editing/text-manipulation/sorting-alignment.md)

