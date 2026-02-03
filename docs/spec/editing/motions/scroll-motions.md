# Scroll Motions

Viewport and cursor motion commands that manipulate scrolling.

## Scope

- Scroll motions MUST preserve the viewport invariants in `/docs/spec/features/ui/viewport.md`.
- Mouse scrolling is out of scope; mouse input is ignored.

## Command classes

| Class | Examples | Cursor changes? | Viewport changes? |
|-------|----------|-----------------|-------------------|
| Reposition viewport around cursor | `zz`, `zt`, `zb`, `z<CR>`, `z.`, `z-` | Sometimes | Yes |
| Scroll viewport (cursor tries to stay) | `Ctrl-e`, `Ctrl-y` | Only if needed to remain visible | Yes |
| Page scroll (cursor moves) | `Ctrl-d`, `Ctrl-u`, `Ctrl-f`, `Ctrl-b` | Yes | Yes |
| Horizontal scroll (no-wrap only) | `zh`, `zl`, `zH`, `zL`, `zs`, `ze` | Only if needed to remain visible | Yes |

## Repositioning (`z` family)

These commands set the viewport position relative to the focused cursor, then apply clamping.

| Key | Requirement |
|-----|-------------|
| `zz` | Cursor display row MUST become `floor(text_rows / 2)` when possible. |
| `zt` | Cursor display row MUST become `0` when possible. |
| `zb` | Cursor display row MUST become `text_rows - 1` when possible. |
| `z<CR>` | Like `zt`, then cursor moves to first non-blank on the cursor line. |
| `z.` | Like `zz`, then cursor moves to first non-blank on the cursor line. |
| `z-` | Like `zb`, then cursor moves to first non-blank on the cursor line. |

## Line scroll (viewport-first)

| Key | Requirement |
|-----|-------------|
| `Ctrl-e` | Viewport MUST move down by one display row when possible. |
| `Ctrl-y` | Viewport MUST move up by one display row when possible. |

After applying `Ctrl-e` / `Ctrl-y`, if the focused cursor would become invisible, the cursor MUST be clamped/moved just enough to remain visible.

## Page scroll (cursor + viewport)

| Key | Amount |
|-----|--------|
| `Ctrl-d` | Half page down (see `/docs/spec/features/ui/scroll-customization.md`) |
| `Ctrl-u` | Half page up (see `/docs/spec/features/ui/scroll-customization.md`) |
| `Ctrl-f` | Full page down (`text_rows`) |
| `Ctrl-b` | Full page up (`text_rows`) |

Requirements:

- The cursor line MUST move by the scroll amount (clamped to buffer bounds).
- The viewport MUST be updated so the focused cursor remains visible.
- The command MUST preserve the cursor column using the active column model from `/docs/spec/editing/cursor/README.md`.

## Horizontal scroll (no-wrap)

Horizontal scroll commands apply only when `wrap = false`.

| Key | Requirement |
|-----|-------------|
| `zh` | Viewport MUST move right by at least `ui.viewport.sidescroll` display columns. |
| `zl` | Viewport MUST move left by at least `ui.viewport.sidescroll` display columns. |
| `zH` | Viewport MUST move right by `floor(text_cols / 2)` columns. |
| `zL` | Viewport MUST move left by `floor(text_cols / 2)` columns. |
| `zs` | Viewport MUST scroll so the cursor is at the left of the text area when possible. |
| `ze` | Viewport MUST scroll so the cursor is at the right of the text area when possible. |

After applying horizontal scroll, the viewport MUST continue to satisfy `ui.viewport.sidescrolloff` when possible.

## Related

- Viewport invariants and follow rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Scroll configuration: [/docs/spec/features/ui/scroll-customization.md](/docs/spec/features/ui/scroll-customization.md)
- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
