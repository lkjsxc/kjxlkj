# File Operations

Back: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

Core file-related ex commands for opening, reading, and managing files.

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

## Error handling

| Error | Behavior |
|---|---|
| File not found (`:e`) | Create new empty buffer with the path |
| Permission denied | Error notification |
| Path is directory | Error notification |
| Binary file | Warning, open with `binary` option set |

## Related

- Write commands: [/docs/spec/commands/file/write-commands.md](/docs/spec/commands/file/write-commands.md)
- Directory commands: [/docs/spec/commands/file/directory-commands.md](/docs/spec/commands/file/directory-commands.md)
- Encoding commands: [/docs/spec/commands/file/encoding-commands.md](/docs/spec/commands/file/encoding-commands.md)
