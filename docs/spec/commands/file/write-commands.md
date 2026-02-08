# Write Commands

Back: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

Commands for saving buffer content to files.

## Basic write

| Command | Description |
|---|---|
| `:w[rite]` | Write current buffer to its associated file |
| `:w[rite] {file}` | Write current buffer to `{file}` |
| `:w[rite]!` | Force write (override read-only, create directories) |
| `:w[rite]! {file}` | Force write to `{file}` |

## Write and quit

| Command | Description |
|---|---|
| `:wq` | Write and close current window |
| `:wq {file}` | Write to `{file}` and close |
| `:wq!` | Force write and close |
| `:x[it]` | Write only if modified, then close |
| `:xa[ll]` / `:wqa[ll]` | Write all modified buffers and quit |

## Partial write

| Command | Description |
|---|---|
| `:{range}w {file}` | Write lines in `{range}` to `{file}` |
| `:{range}w >> {file}` | Append lines in `{range}` to `{file}` |

## Write all

| Command | Description |
|---|---|
| `:wa[ll]` | Write all modified buffers |
| `:wa[ll]!` | Force write all modified buffers |

## Save as

| Command | Description |
|---|---|
| `:sav[eas] {file}` | Write to `{file}` and switch buffer to that file |
| `:sav[eas]! {file}` | Force save-as |

## Write behavior

| Phase | Action |
|---|---|
| Pre-write | Fire `BufWritePre` autocommand |
| Backup | Create backup file if `backup` option is set |
| Write | Write buffer content to file via FS service |
| Post-write | Fire `BufWritePost` autocommand |
| State | Clear modified flag |

## Backup options

| Setting | Default | Description |
|---|---|---|
| `backup` | `false` | Create backup before writing |
| `backupdir` | `~/.local/share/kjxlkj/backup/` | Backup directory |
| `writebackup` | `true` | Create backup during write (removed after success) |

## Encoding

The buffer is written using the encoding specified by the `fileencoding` option. If not set, `utf-8` is used. BOM is written if `bomb` option is set.

## File format

Line endings are written according to the `fileformat` option:

| Value | Line ending |
|---|---|
| `unix` | LF (`\n`) |
| `dos` | CRLF (`\r\n`) |

## Error handling

| Error | Behavior |
|---|---|
| File is read-only | Error unless `!` is used |
| Directory does not exist | Error unless `!` is used (creates directory) |
| Disk full | Error with message |
| Permission denied | Error with message |

## Related

- File operations: [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md)
- Quit commands: [/docs/spec/commands/quit-commands.md](/docs/spec/commands/quit-commands.md)
