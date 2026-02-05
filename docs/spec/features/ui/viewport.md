# Viewport Management

Deterministic rules for mapping buffers to on-screen window regions.

## Scope

- Viewport state is core-owned, per-window.
- The viewport MUST be updated so the focused cursor is visible.
- Mouse-driven scrolling is out of scope; mouse input is ignored.

## Definitions

| Term | Meaning |
|------|---------|
| Window | A viewport over a buffer. |
| Viewport | The window-local scroll state selecting which content is visible. |
| Text area | The window region that can render buffer text (excluding gutters). |
| Display column | A screen column after accounting for tabs and wide graphemes. |
| Display row | A screen row after accounting for folds and soft wrapping. |

## Viewport state (per window)

| Field | Type | Meaning |
|-------|------|---------|
| `top_line` | integer | First buffer line shown at top of text area. |
| `left_col` | integer | First display column shown at left of text area (no-wrap only). |
| `wrap` | boolean | Soft-wrap long lines in the text area width. |
| `text_rows` | integer | Height of the text area in rows. |
| `text_cols` | integer | Width of the text area in columns. |
| `scrolloff` | integer | Desired vertical margin around cursor (rows). |
| `sidescrolloff` | integer | Desired horizontal margin around cursor (cols). |
| `past_end` | enum | Whether the viewport may show virtual empty rows past EOF. |
| `smooth_follow` | boolean | Whether viewport changes may be animated (UI-only). |

## Defaults (normative)

| Field | Default | Notes |
|---|---|---|
| `wrap` | `true` | Long lines wrap by default (Vim-like). When `wrap = true`, `left_col` MUST be `0`. |
| `top_line` | `0` | Clamped per invariants. |
| `left_col` | `0` | Used only when `wrap = false`. |

## Invariants

| Rule | Requirement |
|------|-------------|
| Cursor visibility | After any operation that changes cursor position, folds, wrap, or layout, the focused cursor MUST be visible within the window text area. |
| Determinism | Given identical buffer snapshots, cursor positions, window geometry, and options, viewport updates MUST be identical. |
| Clamping | `top_line` and `left_col` MUST be clamped to valid ranges for the active buffer and window geometry. |
| Wrap interaction | When `wrap = true`, `left_col` MUST be `0` (no horizontal scrolling). |
| Smoothness | If `smooth_follow = true`, smoothing MUST NOT produce any frame where the focused cursor is outside the viewport. |

## Effective margins

The effective margins MUST be clamped to half the viewport to avoid impossible constraints.

| Value | Definition |
|-------|------------|
| `v_margin` | `min(scrolloff, floor((text_rows - 1) / 2))` |
| `h_margin` | `min(sidescrolloff, floor((text_cols - 1) / 2))` |

## Cursor-follow algorithm (normative)

### Vertical follow (no-wrap model)

Given focused cursor line `c_line` and viewport `top_line`:

1. Compute cursor row: `c_row = c_line - top_line`
2. Define row bounds: `min_row = v_margin`, `max_row = text_rows - 1 - v_margin`
3. If `c_row < min_row`, set `top_line := c_line - min_row`
4. If `c_row > max_row`, set `top_line := c_line - max_row`
5. Clamp `top_line` to the buffer’s valid vertical range

### Vertical follow (wrap/fold display-row model)

When `wrap = true` and/or folds are present, the implementation MUST treat each buffer line as producing one or more display rows.

The implementation MUST update `top_line` so the focused cursor’s display row is within `[min_row, max_row]` using a deterministic method.

If the implementation uses an incremental method, it MUST be equivalent to:

1. While cursor display row is above `min_row`, decrement `top_line` by buffer lines until satisfied or `top_line = 0`
2. While cursor display row is below `max_row`, increment `top_line` by buffer lines until satisfied or `top_line` hits its clamp

### Horizontal follow (no-wrap only)

When `wrap = false`, the viewport MUST ensure the focused cursor display column `c_col` is visible.

Given `left_col`:

1. Compute cursor col in viewport: `c_x = c_col - left_col`
2. Define col bounds: `min_x = h_margin`, `max_x = text_cols - 1 - h_margin`
3. If `c_x < min_x`, set `left_col := c_col - min_x`
4. If `c_x > max_x`, set `left_col := c_col - max_x`
5. Clamp `left_col` to the buffer line’s valid horizontal range

### Explicit positioning commands

These commands set the viewport position directly, then apply clamping:

| Command | Requirement |
|---------|-------------|
| Center (`zz`) | Cursor display row MUST become `floor(text_rows / 2)` when possible. |
| Top (`zt`) | Cursor display row MUST become `0` when possible. |
| Bottom (`zb`) | Cursor display row MUST become `text_rows - 1` when possible. |

## Clamping and `past_end`

| `past_end` | Requirement |
|------------|-------------|
| `none` | The viewport MUST clamp so the last visible content row does not exceed EOF (subject to folds/wrap). |
| `scroll` | The viewport MAY show virtual empty rows past EOF, but the cursor MUST still clamp to valid buffer positions per `/docs/spec/editing/cursor/README.md`. |

## Resize handling

When window geometry changes (terminal resize, split changes):

- `text_rows` and `text_cols` MUST be recomputed.
- `top_line` and `left_col` MUST be re-clamped.
- Cursor-follow MUST be re-applied so the focused cursor remains visible.

## Related

- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Windows (viewports over buffers): [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Scroll configuration: [/docs/spec/features/ui/scroll-customization.md](/docs/spec/features/ui/scroll-customization.md)
