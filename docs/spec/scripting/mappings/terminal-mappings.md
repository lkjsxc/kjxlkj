# Terminal Mappings

Back: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)

Key mappings specific to terminal mode.

## Overview

Terminal mode mappings are defined with `tmap` / `tnoremap`. They apply when a terminal buffer is focused.

## Default Mappings

| Key | Action |
|---|---|
| `<C-\><C-n>` | Exit terminal mode to normal mode |
| `<C-w>` | Window command prefix (passes through to editor) |

## Common User Mappings

| Mapping | Purpose |
|---|---|
| `<Esc>` → `<C-\><C-n>` | Use Escape to exit terminal |
| `<C-w>h/j/k/l` → window navigation | Navigate between windows from terminal |

## Key Pass-Through

By default, all keys except `<C-\><C-n>` are passed to the terminal process. User-defined `tmap` entries intercept keys before they reach the terminal.

## Related

- Terminal mode: [/docs/spec/modes/terminal/README.md](/docs/spec/modes/terminal/README.md)
- Mapping modes: [/docs/spec/scripting/mappings/mapping-modes.md](/docs/spec/scripting/mappings/mapping-modes.md)
