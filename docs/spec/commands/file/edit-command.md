# Edit Command

Back: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

The `:edit` command opens files for editing in the current window.

## Scope and Target (normative)

`edit` is a focused-window command.

- it retargets only the focused window buffer binding
- it MUST NOT change buffer bindings in other windows
- it MAY reuse an existing buffer object for the same file path

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

If two windows show the same buffer and one window runs `:e other.txt`, only the
focused window is retargeted. The other window remains on the original buffer.

## Encoding (normative)

| Option | Behavior |
|---|---|
| `++enc={encoding}` | Open the file with the specified encoding (e.g., `++enc=utf-8`, `++enc=latin1`) |
| `++ff={format}` | Set the file format: `unix` (LF), `dos` (CRLF), `mac` (CR) |

## Tab completion

While typing the file argument, `Tab` completes file names from the cwd. Double-tab shows all matches in a completion menu.

## Error Handling (normative)

| Error | Required Behavior |
|---|---|
| pending unsaved changes and no `!` | reject command and keep original focused buffer |
| read failure (permission, decoding, I/O) | keep original focused buffer; surface explicit error |
| invalid `+{command}` payload | reject with usage error; no state mutation |

## Mandatory Verification

| ID | Scenario | Required Assertions |
|---|---|---|
| `CMD-02R` | run `:e` in one pane of two-pane layout | non-focused pane buffer binding is unchanged |
| `FS-01` | run `:e missing-file` | buffer opens as unnamed-on-disk target and writes on first `:w` |
| `FS-02` | run `:e` with unsaved changes and no `!` | command is rejected with no buffer retarget |

## Related

- File operations: [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md)
- Write commands: [/docs/spec/commands/file/write-commands.md](/docs/spec/commands/file/write-commands.md)
- Buffer model: [/docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)
- Execution context: [/docs/spec/commands/execution-context.md](/docs/spec/commands/execution-context.md)

