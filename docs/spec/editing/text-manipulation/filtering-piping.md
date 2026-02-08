# Filtering and Piping

Back: [docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)

Pass buffer text through external commands.

## Overview

Filtering sends buffer text to an external command's
stdin and replaces it with the command's stdout.
This enables using Unix tools (sort, awk, sed, etc.)
directly on buffer content.

## Filter Operator

### Syntax

`!{motion}` filters the lines covered by the motion
through an external command. After typing `!{motion}`,
the command line opens with `:.!` prefix where the
user types the command.

### Line Filter

`!!` filters the current line. `{count}!!` filters
count lines starting from the current.

### Visual Filter

In visual mode, `!` filters the selected lines.
The selection is always expanded to full lines.

## Read Command

### Syntax

`:read !{command}` inserts the output of the command
below the current line.

`:read` without `!` reads from a file instead.

### Line Position

`:{n}read !{command}` inserts output below line n.
`:0read !{command}` inserts at the top of the file.

## Write to Command

### Syntax

`:{range}write !{command}` sends the range of lines
to the command's stdin. The buffer is not modified.

### Without Range

`:write !command` sends the entire buffer to the
command's stdin.

## Common Filter Examples

| Action | Command |
|--------|---------|
| Sort lines | `:%!sort` |
| Sort unique | `:%!sort -u` |
| Format JSON | `:%!jq .` |
| Format Python | `:%!black -` |
| Column extract | `:%!awk '{print $1}'` |
| Reverse lines | `:%!tac` |
| Number lines | `:%!nl` |
| Base64 encode | `:'<,'>!base64` |

## Execution

### Shell

The command runs through the user's shell (`$SHELL`
or `/bin/sh`). Shell features like pipes, redirects,
and expansion are available.

### Working Directory

The command runs in the editor's working directory
(the workspace root).

### Environment

The command inherits the editor's environment
variables. No additional variables are set.

## Error Handling

### Non-Zero Exit

If the command exits with non-zero status, the
filter is aborted. The original text is preserved.
The error output is shown in the message area.

### Stderr

Stderr from the command is captured and displayed
in the message area after the command completes.

### Timeout

External commands have a configurable timeout
(default: 30 seconds). Exceeded timeout kills the
process and preserves original text.

## Undo Behavior

The entire filter operation (replacing text with
command output) is a single undo unit.

## Range Integration

### With Ranges

`:{range}!{command}` filters the specified range.
All range specifiers work: line numbers, marks,
patterns, visual selection.

### Examples

| Range | Meaning |
|-------|---------|
| `:.!cmd` | Filter current line |
| `:%!cmd` | Filter entire buffer |
| `:1,10!cmd` | Filter lines 1-10 |
| `:'<,'>!cmd` | Filter visual selection |
| `:g/pat/!cmd` | Filter matched lines |

## Related

- Range specs: [docs/spec/commands/ranges/range-specs.md](/docs/spec/commands/ranges/range-specs.md)
- Read command: [docs/spec/commands/file/read-command.md](/docs/spec/commands/file/read-command.md)
- Shell integration: [docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
