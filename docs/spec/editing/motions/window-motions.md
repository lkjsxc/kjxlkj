# Window Motions

Back: [/docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)

Navigate cursor to screen positions within the visible window area.

## Motions (normative)

| Key | Name | Target |
|---|---|---|
| `H` | High | First visible line of the window (top) |
| `M` | Middle | Middle visible line of the window |
| `L` | Low | Last visible line of the window (bottom) |

## Behavior rules

Detailed semantics for each window motion.

### H (High)

- Without count: move cursor to first visible line (respecting `scrolloff`).
- With count `N`: move to the Nth line from the top of the window.
- Effective line: `top_line + max(count - 1, scrolloff)`.
- Cursor column: first non-blank character of the target line.

### M (Middle)

- Move cursor to the middle visible line.
- Effective line: `top_line + floor(text_rows / 2)`.
- Cursor column: first non-blank character of the target line.
- Count is ignored.

### L (Low)

- Without count: move cursor to last visible line (respecting `scrolloff`).
- With count `N`: move to the Nth line from the bottom of the window.
- Effective line: `top_line + text_rows - 1 - max(count - 1, scrolloff)`.
- Cursor column: first non-blank character of the target line.

## Scrolloff interaction

When `scrolloff` is set (e.g., to 5):

| Motion | Without scrolloff | With scrolloff=5 |
|---|---|---|
| `H` | Line 1 of window | Line 6 of window |
| `L` | Last line of window | 5th line from bottom |

## Operator compatibility

H, M, L can be used as motions with operators:

| Example | Effect |
|---|---|
| `dH` | Delete from cursor to first visible line |
| `yL` | Yank from cursor to last visible line |
| `cM` | Change from cursor to middle visible line |

These motions are linewise when used with operators.

## Jump list behavior

H, M, L do NOT add entries to the jump list. They are considered minor cursor movements.

## Comparison with file motions

| Motion | Scope | Scrolls viewport |
|---|---|---|
| `gg` | File (first line) | Yes |
| `G` | File (last line) | Yes |
| `H` | Window (top visible) | No |
| `M` | Window (middle) | No |
| `L` | Window (bottom) | No |

## Related

- Line motions: [/docs/spec/editing/motions/line-motions.md](/docs/spec/editing/motions/line-motions.md)
- Scroll motions: [/docs/spec/editing/motions/scroll-motions.md](/docs/spec/editing/motions/scroll-motions.md)
- Viewport: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
