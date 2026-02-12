# File Operations

Back: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

Core file-related ex commands for opening, reading, and managing files.

## Scope Rule (normative)

File operations default to focused-window scope unless explicit-global command is
used.

Global exceptions:

- `:wa`, `:wa!`
- `:wqa`, `:xa`

## Open file

| Command | Description |
|---|---|
| `:e[dit] {file}` | Open `{file}` in current window |
| `:e[dit]` | Re-read current file from disk |
| `:e[dit]!` | Re-read current file, discarding changes |
| `:ene[w]` | Open a new empty buffer |

## Read file

| Command | Description |
|---|---|
| `:r[ead] {file}` | Insert contents of `{file}` below cursor |
| `:r[ead] !{cmd}` | Insert output of shell command below cursor |
| `:{n}r {file}` | Insert below line `{n}` |

## Save as

| Command | Description |
|---|---|
| `:sav[eas] {file}` | Save buffer to `{file}` and switch to it |
| `:sav[eas]! {file}` | Force save-as |

## File information

| Command | Description |
|---|---|
| `:f[ile]` | Show current file name, modification status, position |
| `:f[ile] {name}` | Set the buffer file name |
| `Ctrl-g` | Show file info in statusline |

## File encoding

| Command | Description |
|---|---|
| `:set fileencoding={enc}` | Set encoding for current buffer |
| `:set fileformat={ff}` | Set line ending format (`unix`, `dos`) |

## New from template

| Command | Description |
|---|---|
| `:e {file}` | If file doesn't exist, create a new buffer with that path |

## FS service interaction

All file operations dispatch to the FS service:

| Operation | FS service message |
|---|---|
| `:e {file}` | `FileRead { path }` |
| `:w` | `FileWrite { path, content }` |
| `:r {file}` | `FileRead { path }` (content inserted into buffer) |
| `:sav {file}` | `FileWrite { path, content }` then buffer rename |

The core task sends the message and awaits the response asynchronously.

Each request MUST include a deterministic request ID and target `BufferId`.
Responses MUST be matched by request ID before state mutation.

## Error handling

| Error | Behavior |
|---|---|
| File not found (`:e`) | Create new empty buffer with the path |
| Permission denied | Error notification |
| Path is directory | Error notification |
| Binary file | Warning, open with `binary` option set |

On any FS failure:

- do not clear modified flag
- do not partially apply content
- keep focused window/buffer binding unchanged unless command explicitly allows fallback creation

## Mandatory Verification

| ID | Scenario | Required Assertions |
|---|---|---|
| `FS-03R` | edit, modify, write, reopen | content round-trip is exact |
| `FS-04` | permission-denied write | no data loss and modified flag unchanged |
| `FS-07` | failed read | no state mutation on error |

## Related

- Write commands: [/docs/spec/commands/file/write-commands.md](/docs/spec/commands/file/write-commands.md)
- Directory commands: [/docs/spec/commands/file/directory-commands.md](/docs/spec/commands/file/directory-commands.md)
- Encoding commands: [/docs/spec/commands/file/encoding-commands.md](/docs/spec/commands/file/encoding-commands.md)
- Execution context: [/docs/spec/commands/execution-context.md](/docs/spec/commands/execution-context.md)
