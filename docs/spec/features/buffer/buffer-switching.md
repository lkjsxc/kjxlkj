# Buffer Switching

Back: [/docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md)

Strategies and commands for navigating between open buffers.

## Direct access (normative)

| Command / Key | Action | Detail |
|---|---|---|
| `:b {N}` | Switch to buffer number N | Buffer numbers are stable across the session |
| `:b {name}` | Switch to buffer matching name | Partial match; error if ambiguous |
| `:b {path}` | Switch to buffer by file path | Matches against absolute or relative path |
| `{N}Ctrl-^` | Switch to buffer N | From Normal mode |

## Sequential navigation (normative)

| Command / Key | Action | Detail |
|---|---|---|
| `:bnext` / `:bn` | Next buffer in list | Wraps around to first buffer after last |
| `:bprev` / `:bp` | Previous buffer in list | Wraps around to last buffer before first |
| `:bfirst` / `:bf` | First buffer in list | |
| `:blast` / `:bl` | Last buffer in list | |

Count prefix: `:3bnext` skips 3 buffers forward.

## Alternate file (normative)

| Command / Key | Action |
|---|---|
| `Ctrl-^` | Toggle between current and alternate buffer |
| `:b #` | Switch to alternate buffer |

The alternate buffer is the previously active buffer. It is updated whenever the user switches buffers via any method.

## Modified buffer guard (normative)

Switching away from a modified buffer:

| Scenario | Behavior |
|---|---|
| `:b` to another buffer, current is modified | Error: "No write since last change (add ! to override)" |
| `:b!` to another buffer, current is modified | Switch without saving; modified buffer remains in list |
| `set hidden` | All buffer switches succeed without `!`; modified buffers persist in background |

The `hidden` option (default: true) suppresses the modified guard for most operations.

## Fuzzy buffer picker

The finder (`:Finder buffers` or `<leader>fb`) opens a fuzzy picker showing:

| Column | Content |
|---|---|
| Number | Buffer number |
| Flags | `%a` (active), `#` (alternate), `+` (modified), `-` (readonly) |
| Name | File name or `[Scratch]` |
| Path | Relative path from cwd |

Typing filters by fuzzy match on name and path. `Enter` opens the selected buffer.

## MRU ordering

Buffer navigation commands can optionally use most-recently-used order instead of buffer number order. Configuration: `buffer.mru_order = true`.

When MRU is enabled, `:bnext` moves to the next most recently accessed buffer rather than the next numbered buffer.

## Related

- Buffer model: [/docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)
- Buffer listing: [/docs/spec/commands/buffer/buffer-listing.md](/docs/spec/commands/buffer/buffer-listing.md)
- Alternate file: [/docs/spec/features/buffer/alternate-file.md](/docs/spec/features/buffer/alternate-file.md)
- Bufferline: [/docs/spec/features/buffer/bufferline.md](/docs/spec/features/buffer/bufferline.md)
