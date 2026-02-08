# Read Command

Back: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

Insert file contents or command output into the buffer.

## Overview

`:read` (`:r`) inserts text into the current buffer from a file or from the output of a shell command.

## File Read

`:r {file}` — insert contents of `{file}` below the current line.

`:0r {file}` — insert at the top of the buffer.

## Command Read

`:r !{cmd}` — execute `{cmd}` and insert its stdout below the current line.

| Example | Effect |
|---|---|
| `:r !date` | Insert current date |
| `:r !ls` | Insert directory listing |
| `:r !sort %` | Insert sorted version of current file |

## Range

`:10r {file}` — insert after line 10.

`:{range}r {file}` — insert after the last line of the range.

## Related

- File operations: [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md)
- Write commands: [/docs/spec/commands/file/write-commands.md](/docs/spec/commands/file/write-commands.md)
