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

kjxlkj queries the OS for the active keyboard layout at startup:

| Platform | Detection Method |
|----------|-----------------|
| Linux (X11) | `setxkbmap -query` |
| Linux (Wayland) | `localectl status` or compositor API |
| macOS | `defaults read ~/Library/Preferences/com.apple.HIToolbox` |
| Windows | Win32 `GetKeyboardLayout` API |

If detection succeeds, the layout is applied automatically.
Override with an explicit setting if detection is wrong.

### Explicit Layout

Set the layout directly in the config file:

```
set keyboardlayout=dvorak
```

| Value | Layout |
|-------|--------|
| `qwerty` | US QWERTY (default) |
| `dvorak` | Simplified Dvorak |
| `colemak` | Colemak |
| `workman` | Workman |
| `custom` | User-defined mapping (see below) |

### Remap for Layout

Some users prefer vim keys to stay in QWERTY positions:

```
set langmap=dh,hj,tk,nl
```

This works like Vim's `langmap`: the left character (physical key)
is translated to the right character (logical key) in Normal mode.
Insert mode is not affected, so typing remains natural.

With Dvorak, this maps:
- Physical `h` position -> left
- Physical `j` position -> down
- Physical `k` position -> up
- Physical `l` position -> right

### Custom Key Remapping

Define a full remapping table for any layout:

```
[keymap.custom]
langmap = """
  dh,hj,tk,nl,
  DH,HJ,TK,NL,
  -[,=[,_],+]
"""
```

| Field | Purpose |
|-------|---------|
| `langmap` | Comma-separated `from`/`to` character pairs |

Each pair maps a physical key to a logical key. Uppercase
pairs handle shifted variants.

## Dvorak Considerations

Common Dvorak-friendly remappings:

| QWERTY | Dvorak Physical | Suggested |
|--------|-----------------|-----------|
| `h` | `d` | Keep or remap |
| `j` | `h` | Keep or remap |
| `k` | `t` | Keep or remap |
| `l` | `n` | Keep or remap |

### Full Dvorak Config

```
set keyboardlayout=dvorak
set langmap=dh,hj,tk,nl,DH,HJ,TK,NL
-- Optionally remap common operators too:
nmap s cursor.down      -- Dvorak 's' is easy to reach
nmap n search.next      -- restore 'n' to search-next
```

## Colemak Considerations

Colemak has `h`, `j`, `k` close to QWERTY but `l` moves:

| QWERTY | Colemak Physical | Change |
|--------|------------------|--------|
| `h` | `h` | Same position |
| `j` | `n` | Moved (QWERTY `n` position) |
| `k` | `e` | Moved (QWERTY `e` position) |
| `l` | `i` | Moved (QWERTY `i` position) |

Popular Colemak remapping (`set langmap=hn,ne,ei,il`):
- `h` stays as left
- `n` becomes down, `e` becomes up, `i` becomes right

## Layer-aware Keyboards

For keyboards with layers (like QMK):
- Configure layers in firmware
- kjxlkj receives standard keycodes
- No special configuration needed

For compose keys and dead keys:
- The terminal handles composition before kjxlkj sees input
- kjxlkj receives the final composed character (e.g., `e` + `'` -> `e`)
- If a dead key does not compose, kjxlkj treats it as a literal

| Scenario | Behavior |
|----------|----------|
| QMK / ZMK layers | Transparent; firmware resolves layers |
| Compose sequences | Terminal resolves; editor sees result |
| Dead keys (e.g., `'` + `e`) | Terminal composes; editor sees `e` |
| AltGr characters | Terminal sends Unicode; editor receives it |

## Troubleshooting

### Keys not working as expected

1. Check terminal key reporting: press keys in insert mode
2. Verify layout setting matches actual layout
3. Check for conflicting remaps

### International Layouts

For layouts with dead keys or AltGr:
- Use terminal that handles composition
- kjxlkj receives composed characters
