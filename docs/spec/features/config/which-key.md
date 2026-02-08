# Which-Key Style Hints

kjxlkj displays keybinding hints as you type.

## Overview

When you start a key sequence, a popup shows available
completions after a configurable delay. This helps discover
keybindings and prevents timeouts on complex sequences.

## Configuration

Under `[which_key]` in config TOML:

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enable` | bool | `true` | Enable which-key popup |
| `delay` | integer | `500` | Delay in ms before popup appears |
| `max_height` | integer | `10` | Maximum popup height in lines |
| `position` | string | `"bottom"` | Popup position |
| `separator` | string | `" -> "` | Separator between key and description |
| `sort` | string | `"alpha"` | Sort order: `"alpha"`, `"order"` |

### Popup Position

| Value | Position |
|-------|----------|
| `"bottom"` | Bottom of screen, above statusline |
| `"center"` | Centered floating window |
| `"cursor"` | Near cursor position |

## Key Descriptions

Add descriptions to keybindings in TOML:
Under `[keys.normal]`, each mapping can have a `desc` field
that appears in the which-key popup.

## Grouping

Create logical groups by using common prefixes.
`<Leader>f` for file operations, `<Leader>g` for git, etc.
The popup shows group names when a prefix is typed.

Group display:
- `f` -> `+file`
- `g` -> `+git`
- `b` -> `+buffer`

Each group expands to show its members when selected.

## Hiding Keys

Hide specific keys from hints by setting `hidden = true`
on the mapping. Internal or rarely-used mappings can be
hidden to reduce clutter.

## Appearance

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `border` | string | `"rounded"` | Border style |
| `width` | string | `"auto"` | Width: `"auto"`, integer, or percentage |
| `highlight` | string | `"WhichKey"` | Highlight group for keys |
| `desc_highlight` | string | `"WhichKeyDesc"` | Highlight for descriptions |
| `separator_highlight` | string | `"WhichKeySep"` | Highlight for separators |

## Trigger Keys

Which-key popup triggers after these keys by default:

| Key | Context |
|-----|---------|
| `<Leader>` | Leader key sequence |
| `g` | Go-to commands |
| `z` | Fold/scroll commands |
| `"` | Register selection |
| `'` | Mark jump |
| `` ` `` | Mark jump (exact position) |
| `<C-w>` | Window commands |
| `[` | Previous navigation |
| `]` | Next navigation |

Additional triggers can be configured.

## Integration

### With Leader Key

The which-key popup is most useful with `<Leader>`.
After pressing `<Leader>`, all registered leader mappings
are shown grouped by category.

### With Operators

Operator pending mode also shows hints. After `d`, the
popup shows available motions and text objects:
- `w` -> word
- `$` -> end of line
- `iw` -> inner word

Followed by motion hints showing the pending operation.
