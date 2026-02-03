# Scroll Customization

Configuration for viewport follow and scroll behavior.

## Scope

- Scroll customization affects per-window viewport behavior.
- Mouse scrolling is out of scope; mouse input is ignored.

## Configuration keys (normative)

| Key | Type | Default | Meaning |
|-----|------|---------|---------|
| `ui.viewport.scrolloff` | integer | `0` | Vertical context margin around cursor (rows). |
| `ui.viewport.sidescrolloff` | integer | `0` | Horizontal context margin around cursor (cols, no-wrap only). |
| `ui.viewport.scroll` | integer | `0` | Default line count for half-page scroll commands when no count is provided. |
| `ui.viewport.sidescroll` | integer | `1` | Minimum horizontal scroll amount (cols) for horizontal scroll commands. |
| `ui.viewport.past_end` | enum | `none` | Whether viewport may show virtual rows past EOF (`none`, `scroll`). |
| `ui.viewport.smooth_follow` | boolean | `false` | Whether viewport motion may be animated (UI-only). |

## Semantics

### Cursor-follow margins

- The viewport MUST apply margins as defined in `/docs/spec/features/ui/viewport.md`.
- If the margin cannot be satisfied (near edges, tiny windows), the viewport MUST clamp deterministically.

### Half-page scroll amount

For `Ctrl-d` and `Ctrl-u`:

- If the user provides an explicit count, the count MUST be used.
- Otherwise:
  - If `ui.viewport.scroll > 0`, that value MUST be used.
  - If `ui.viewport.scroll = 0`, the command MUST scroll by `floor(text_rows / 2)`.

### Horizontal scroll amount (no-wrap)

For horizontal scroll commands (`zh`, `zl`, `zH`, `zL`, `zs`, `ze`):

- The implementation MUST treat `ui.viewport.sidescroll` as a minimum scroll step.
- The viewport MUST continue to satisfy `ui.viewport.sidescrolloff` after the scroll.

## Scroll-related keybindings

These keybindings are defined in `/docs/spec/ux/keybindings/navigation.md` and `/docs/spec/editing/motions/scroll-motions.md`:

| Key | Action |
|-----|--------|
| `Ctrl-e` | Scroll down one line (viewport only) |
| `Ctrl-y` | Scroll up one line (viewport only) |
| `Ctrl-d` | Scroll down half page |
| `Ctrl-u` | Scroll up half page |
| `Ctrl-f` | Scroll down full page |
| `Ctrl-b` | Scroll up full page |
| `zz` | Center cursor line |
| `zt` | Put cursor line at top |
| `zb` | Put cursor line at bottom |

## Related

- Viewport invariants and follow rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Scroll motions: [/docs/spec/editing/motions/scroll-motions.md](/docs/spec/editing/motions/scroll-motions.md)
