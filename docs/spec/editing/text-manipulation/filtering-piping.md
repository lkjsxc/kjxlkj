# Filtering and Piping

Back: [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)

Pass buffer text through external commands.

## Overview

The `!` operator pipes text through an external command, replacing the original text with the command's output.

## Operator

`!{motion}` — filter the text covered by `{motion}` through an external command.

## Line Filter

`!!{cmd}` — filter the current line through `{cmd}`.

`{N}!!{cmd}` — filter N lines through `{cmd}`.

## Range Filter

`:{range}!{cmd}` — filter the range through `{cmd}`.

## Examples

| Command | Effect |
|---|---|
| `:%!sort` | Sort all lines |
| `:%!uniq` | Remove duplicate adjacent lines |
| `!ip sort` | Sort current paragraph |
| `:'<,'>!column -t` | Align visual selection into columns |

## Behavior

1. Selected text is written to the command's stdin.
2. The command's stdout replaces the selected text.
3. If the command exits with non-zero, the text is still replaced (show warning).

## Related

- Text manipulation: [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)
- Read command: [/docs/spec/commands/file/read-command.md](/docs/spec/commands/file/read-command.md)
