# User Experience

Keyboard-only modal editing with async transparency.

## UX Pillars

| Pillar | Requirement |
|--------|-------------|
| Unified UI | Consistent patterns across all features |
| Async transparency | Background work visible (progress, errors) |
| Keyboard-only | No mouse; everything via keys |
| Low-latency | Input always responsive |

## Documentation

| Document | Content |
|----------|---------|
| [keybindings.md](keybindings.md) | Complete keybinding reference |
| [keybindings/](keybindings/README.md) | Keybinding subdocs |
| [layout.md](layout.md) | UI layout specification |
| [theming.md](theming.md) | Theme configuration |
| [accessibility.md](accessibility.md) | Accessibility features |
| [keybinding-dsl.md](keybinding-dsl.md) | Keybinding DSL |
| [keyboard-layouts.md](keyboard-layouts.md) | Keyboard layouts |

## Keybinding Highlights

| Key | Action |
|-----|--------|
| `<leader>e` | File Explorer |
| `<leader>t` | Terminal |
| `<leader>f` | Fuzzy Finder |
| `A` (Shift+a) | Append at line end |
| `I` (Shift+i) | Insert at line start |
| `Ctrl-w s/v` | Split horizontal/vertical |

## UI Elements

| Element | Purpose |
|---------|---------|
| Statusline | Mode, file, position, async status |
| Sidebar | File explorer, symbols |
| Bottom panel | Terminal, diagnostics |
| Overlays | Pickers, confirmations |

## Leader Key

Default: `Space`

All feature shortcuts use leader prefix.

## Related

- [Keybindings](keybindings.md)
- [Features](docs/spec/features/README.md)
