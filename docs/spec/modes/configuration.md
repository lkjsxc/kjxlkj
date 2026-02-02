# Mode Configuration

Mode configuration affects presentation, not editing semantics.

## Requirements

- Configuration MUST NOT change editing semantics
- Cursor style and line number settings are UI concerns only
- Mode transitions remain deterministic regardless of configuration

## Configurable Elements

| Element | Options | Default |
|---------|---------|---------|
| Cursor shape | Block, bar, underline | Block (Normal), Bar (Insert) |
| Cursor blink | On, off | Off |
| Line numbers | Absolute, relative, hybrid | Absolute |
| Status line | Mode indicator format | Uppercase mode name |

## Invariants

1. Keybindings execute identically regardless of cursor style
2. Line number format does not affect navigation commands
3. Mode indicator is always visible in status line

## Related

- UX/theming: [docs/spec/ux/README.md](docs/spec/ux/README.md)
- Cursor semantics: [docs/spec/editing/cursor/README.md](docs/spec/editing/cursor/README.md)
