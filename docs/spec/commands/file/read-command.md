# Read Command

Back: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

Insert file contents or command output into the buffer.

## Scope and Target (normative)

`read` is a focused-window command.

- insertion target is the focused window's current buffer
- insertion point is derived from focused window cursor/range context
- other windows are unaffected except when they share the same buffer object

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

## Error Handling

| Error | Required Behavior |
|---|---|
| source path not found | no buffer mutation; show explicit error |
| command execution error (`:r !cmd`) | no buffer mutation; show command failure |
| decode failure | no partial insert; show decode error |

## Mandatory Verification

| ID | Scenario | Required Assertions |
|---|---|---|
| `FS-05` | `:r file` in split layout | insertion affects focused buffer only |
| `FS-06` | `:r !echo x` | command output inserts deterministically below target line |
| `FS-07` | failed `:r missing.txt` | buffer content and cursor remain unchanged |

## Related

- File operations: [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md)
- Write commands: [/docs/spec/commands/file/write-commands.md](/docs/spec/commands/file/write-commands.md)
- Execution context: [/docs/spec/commands/execution-context.md](/docs/spec/commands/execution-context.md)
