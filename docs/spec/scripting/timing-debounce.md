# Timing and Debounce

Key timing, debounce, and throttle configuration.

## Key Sequence Timeout

| Setting | Default | Description |
|---|---|---|
| `timeout` | `true` | Enable timeout for mapped key sequences |
| `timeoutlen` | `1000` | Milliseconds to wait for next key in mapping |
| `ttimeout` | `true` | Enable timeout for terminal key codes |
| `ttimeoutlen` | `50` | Milliseconds to wait for terminal escape sequence |

When `g` is pressed and a `gx` mapping exists, the editor waits `timeoutlen` ms for the next key. If no key arrives, execute `g` alone (if mapped) or do nothing.

## Debounce

Delay execution until input stops for a specified duration. The timer resets on each new event.

| Feature | Default ms | Purpose |
|---|---|---|
| Incremental search | 150 | Wait for typing to settle |
| Completion trigger | 50 | Fast response after pause |
| Diagnostic display | 100 | Reduce UI flicker |
| Auto-save | 1000 | Batch rapid edits |
| CursorHold event | 4000 | Idle detection |

## Throttle

Limit execution rate to at most once per interval, regardless of input frequency.

| Feature | Default ms | Purpose |
|---|---|---|
| Rendering frames | 16 | 60 fps cap |
| Statusline update | 100 | Reduce redraws |
| Scroll events | 16 | Smooth scrolling |
| Resize handling | 50 | Batch resize events |

## CursorHold

The `CursorHold` event fires after `updatetime` ms of inactivity in normal mode. Used for hover documentation, diagnostic popups, and auto-save triggers.

Default `updatetime`: 4000 ms.

## Configuration

All timing values are configurable in TOML:

| Key | Type | Default |
|---|---|---|
| `editor.timeoutlen` | integer (ms) | 1000 |
| `editor.ttimeoutlen` | integer (ms) | 50 |
| `editor.updatetime` | integer (ms) | 4000 |
| `completion.debounce` | integer (ms) | 50 |
| `search.debounce` | integer (ms) | 150 |

## Related

- Key input: [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- Keybindings: [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)
