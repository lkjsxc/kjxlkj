# Keyboard Layout Handling

kjxlkj supports alternative keyboard layouts.

## Supported Layouts

- QWERTY (default)
- Dvorak
- Colemak
- Workman
- Custom mappings

## Configuration

### Auto-detect System Layout


### Explicit Layout


### Remap for Layout

Some users prefer vim keys to stay in QWERTY positions:


With Dvorak, this maps:
- Physical `h` position → left
- Physical `j` position → down
- Physical `k` position → up
- Physical `l` position → right

### Custom Key Remapping


## Dvorak Considerations

Common Dvorak-friendly remappings:

| QWERTY | Dvorak Physical | Suggested |
|--------|-----------------|-----------|
| `h` | `d` | Keep or remap |
| `j` | `h` | Keep or remap |
| `k` | `t` | Keep or remap |
| `l` | `n` | Keep or remap |

### Full Dvorak Config


## Colemak Considerations

Colemak has `h`, `j`, `k` close to QWERTY but `l` moves:


## Layer-aware Keyboards

For keyboards with layers (like QMK):
- Configure layers in firmware
- kjxlkj receives standard keycodes
- No special configuration needed

## Troubleshooting

### Keys not working as expected

1. Check terminal key reporting: press keys in insert mode
2. Verify layout setting matches actual layout
3. Check for conflicting remaps

### International Layouts

For layouts with dead keys or AltGr:
- Use terminal that handles composition
- kjxlkj receives composed characters
