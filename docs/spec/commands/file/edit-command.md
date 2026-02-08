# Edit Command

Back: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

The `:edit` command opens files for editing in the current window.

## Syntax (normative)

| Command | Action |
|---|---|
| `:e {file}` | Open `{file}` in the current window |
| `:e` | Re-read the current file from disk (reload) |
| `:e!` | Discard changes and re-read from disk |
| `:e! {file}` | Discard changes in current buffer and open `{file}` |
| `:e +{line} {file}` | Open `{file}` and jump to line `{line}` |
| `:e +/{pattern} {file}` | Open `{file}` and search for `{pattern}` |
| `:e +{command} {file}` | Open `{file}` and execute `{command}` |

## Path resolution (normative)

| Path form | Resolution |
|---|---|
| Relative path | Resolved from cwd |
| Absolute path | Used as-is |
| `~/{path}` | Expanded to user home directory |
| `%` | Current file path |
| `#` | Alternate file path |
| Glob patterns | Tab-completion expands globs; not used in `:e` directly |

## Buffer behavior (normative)

| Scenario | Behavior |
|---|---|
| Current buffer is unmodified | Buffer is replaced in the window |
| Current buffer is modified | Error: "No write since last change" (use `:e!` to force) |
| `hidden` option is set | Current buffer moves to background without error |
| File does not exist | Create a new buffer with that path; file created on first `:w` |
| File is already open in a buffer | Reuse the existing buffer (do not create a duplicate) |

## Encoding (normative)

| Option | Behavior |
|---|---|
| `++enc={encoding}` | Open the file with the specified encoding (e.g., `++enc=utf-8`, `++enc=latin1`) |
| `++ff={format}` | Set the file format: `unix` (LF), `dos` (CRLF), `mac` (CR) |

## Tab completion

While typing the file argument, `Tab` completes file names from the cwd. Double-tab shows all matches in a completion menu.

## Related

- File operations: [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md)
- Write commands: [/docs/spec/commands/file/write-commands.md](/docs/spec/commands/file/write-commands.md)
- Buffer model: [/docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)


