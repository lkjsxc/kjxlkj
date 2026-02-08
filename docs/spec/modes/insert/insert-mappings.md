# Insert Mode Mappings

Back: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)

Custom key bindings active in Insert mode.

## Mapping definition (normative)

Insert-mode mappings are defined with the `imap` (recursive) or `inoremap` (non-recursive) command:

| Command | Description |
|---|---|
| `:imap {lhs} {rhs}` | Map `{lhs}` to `{rhs}` in Insert mode (recursive) |
| `:inoremap {lhs} {rhs}` | Map `{lhs}` to `{rhs}` in Insert mode (non-recursive) |
| `:iunmap {lhs}` | Remove Insert-mode mapping for `{lhs}` |
| `:imapclear` | Remove all Insert-mode mappings |

## TOML configuration equivalent

Mappings can also be set in `config.toml` under `[mappings.insert]`:

| TOML key | Value | Equivalent command |
|---|---|---|
| `"jk"` | `"<Esc>"` | `:inoremap jk <Esc>` |
| `"<C-s>"` | `":w<CR>"` | `:inoremap <C-s> <Esc>:w<CR>a` |

## Key notation in lhs

Insert-mode mappings support the same key notation as Normal mode:

| Notation | Key |
|---|---|
| `<C-x>` | Ctrl + x |
| `<M-x>` | Alt + x |
| `<S-Tab>` | Shift + Tab |
| `<CR>` | Enter |
| `<BS>` | Backspace |
| `<Esc>` | Escape |

## Mapping vs literal input

When a mapping's `{lhs}` is a prefix of ongoing input, the keybinding resolver waits for `timeoutlen` milliseconds before deciding. See [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md).

## Interaction with IME

During IME composition (e.g., Japanese input), all Insert-mode mappings MUST be suspended. Only after the IME commits or cancels does mapping resolution resume.

## Buffer-local mappings

The `<buffer>` flag restricts a mapping to the current buffer:

| Command | Scope |
|---|---|
| `:inoremap <buffer> {lhs} {rhs}` | Current buffer only |

Buffer-local mappings take priority over global mappings for the same `{lhs}`.

## Related

- Mapping modes: [/docs/spec/scripting/mappings/mapping-modes.md](/docs/spec/scripting/mappings/mapping-modes.md)
- Input decoding: [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- Insert-mode overview: [/docs/spec/modes/insert/insert.md](/docs/spec/modes/insert/insert.md)

