# Special Keys in Mappings

Key notation reference for keybinding configuration.

## Modifier Keys

| Modifier | Notation | Example |
|---|---|---|
| Control | `<C-x>` | `<C-s>` |
| Alt/Meta | `<M-x>` or `<A-x>` | `<M-a>` |
| Shift | `<S-x>` | `<S-Tab>` |
| Super/Cmd | `<D-x>` | `<D-s>` |

Modifiers may be combined: `<C-S-a>` means Ctrl+Shift+A.

## Function Keys

| Key | Notation |
|---|---|
| F1 through F12 | `<F1>` .. `<F12>` |
| Shift+F1 | `<S-F1>` |
| F13 through F24 | `<F13>` .. `<F24>` |

## Navigation Keys

| Key | Notation |
|---|---|
| Up / Down / Left / Right | `<Up>` `<Down>` `<Left>` `<Right>` |
| Page Up / Page Down | `<PageUp>` `<PageDown>` |
| Home / End | `<Home>` `<End>` |

## Editing Keys

| Key | Notation | Aliases |
|---|---|---|
| Enter | `<CR>` | `<Enter>`, `<Return>` |
| Tab | `<Tab>` | |
| Backspace | `<BS>` | `<Backspace>` |
| Delete | `<Del>` | `<Delete>` |
| Escape | `<Esc>` | |
| Space | `<Space>` | |
| Insert | `<Insert>` | |

## Special Character Notation

| Character | Notation |
|---|---|
| `<` | `<lt>` |
| `>` | `<gt>` |
| `\` | `<Bslash>` |
| `|` | `<Bar>` |

## Leader Keys

`<Leader>` expands to the value of `mapleader` (default: `\`). Set in config:

`<LocalLeader>` expands to `maplocalleader`, used for buffer-local mappings.

## No-Op Key

`<Nop>` disables a key. Mapping a key to `<Nop>` makes it do nothing.

## Plug and SID Keys

`<Plug>` is used for plugin mapping namespaces. Not a real key — serves as a unique prefix to avoid conflicts.

`<SID>` prefixes script-local mappings.

## Mouse Keys (Ignored)

Mouse input is ignored at runtime. The following notation exists only for compatibility and MUST NOT trigger actions:

`<LeftMouse>`, `<RightMouse>`, `<MiddleMouse>`, `<ScrollWheelUp>`, `<ScrollWheelDown>`, `<LeftDrag>`, `<LeftRelease>`

## Terminal Escape Codes

Raw terminal escape sequences are decoded into the key notation above by the input-decoding layer. Mappings always use the human-readable notation.

## Related

- Keybindings: [/docs/spec/modes/keybindings.md](/docs/spec/modes/keybindings.md)
- Input decoding: [/docs/spec/technical/input-decoding.md](/docs/spec/technical/input-decoding.md)
