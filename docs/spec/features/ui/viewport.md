# Viewport Management

Back: [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

Deterministic rules for mapping buffers to on-screen window regions.

## Scope

- Viewport state is core-owned, per-window.
- The viewport MUST be updated so the focused cursor is visible.
- Mouse-driven scrolling is out of scope; mouse input is ignored.

## Definitions

| Term | Meaning |
|---|---|
| Window | A viewport over a content source (buffer or terminal). |
| Viewport | The window-local scroll state selecting which content is visible. |
| Text area | The window region that can render buffer text (excluding gutters). |
| Display column | A screen column after accounting for tabs and wide graphemes. |
| Display row | A screen row after accounting for folds and soft wrapping. |

## Viewport state (per window)

| Field | Type | Meaning |
|---|---|---|
| `top_line` | integer | First buffer line shown at top of text area. |
| `left_col` | integer | First display column shown at left (no-wrap only). |
| `wrap` | boolean | Soft-wrap long lines in the text area width. |
| `text_rows` | integer | Height of the text area in rows. |
| `text_cols` | integer | Width of the text area in columns. |
| `scrolloff` | integer | Desired vertical margin around cursor (rows). |
| `sidescrolloff` | integer | Desired horizontal margin around cursor (cols). |
| `past_end` | enum | Whether virtual empty rows past EOF are allowed. |

## Defaults (normative)

| Field | Default | Notes |
|---|---|---|
| `wrap` | `true` | Long lines wrap by default. When `wrap = true`, `left_col` MUST be `0`. |
| `top_line` | `0` | Clamped per invariants. |
| `left_col` | `0` | Used only when `wrap = false`. |

## Invariants

| Rule | Requirement |
|---|---|
| Cursor visibility | After any cursor/layout change, the focused cursor MUST be visible. |
| Determinism | Same inputs MUST produce identical viewport updates. |
| Clamping | `top_line` and `left_col` MUST stay in valid ranges. |
| Wrap interaction | When `wrap = true`, `left_col` MUST be `0`. |

## Line wrapping algorithm (normative)

When `wrap = true`, any line whose total display width exceeds `text_cols` MUST wrap to continuation display rows.

### Wrapping procedure

For each buffer line, compute display rows as follows:

1. Decompose the line into grapheme clusters with their display widths (per [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)).
2. Fill display columns left to right. When adding a grapheme would exceed `text_cols`, start a new display row.
3. Wide characters (width 2): if only 1 column remains on the current display row, the wide character MUST start on the next display row. The remaining column on the current row MUST be rendered as a padding cell (empty or filler character), not as the first half of the wide character.
4. Each buffer line produces `ceil(total_display_width / text_cols)` display rows (minimum 1).

### Wide character wrap-boundary rule (normative)

When a width-2 grapheme would be split across two display rows (i.e., only 1 column remains on the current row):

| Action | Requirement |
|---|---|
| Current row | MUST render a 1-column padding cell at the remaining position. |
| Next row | MUST start with the full width-2 grapheme. |

This prevents a CJK character from being rendered with its first column on one row and second column on the next row.

### No-wrap mode

When `wrap = false`, lines extend beyond `text_cols` and the viewport scrolls horizontally. The cursor display column MUST account for wide characters using actual display widths, not byte or code point counts.

## Effective margins

| Value | Definition |
|---|---|
| `v_margin` | `min(scrolloff, floor((text_rows - 1) / 2))` |
| `h_margin` | `min(sidescrolloff, floor((text_cols - 1) / 2))` |

## Cursor-follow algorithm (normative)

### Vertical follow (no-wrap)

Given focused cursor line `c_line` and viewport `top_line`:

1. `c_row = c_line - top_line`
2. `min_row = v_margin`, `max_row = text_rows - 1 - v_margin`
3. If `c_row < min_row`, set `top_line := c_line - min_row`
4. If `c_row > max_row`, set `top_line := c_line - max_row`
5. Clamp `top_line` to valid range.

### Vertical follow (wrap/fold display-row model)

When `wrap = true`, the implementation MUST treat each buffer line as producing one or more display rows.

The implementation MUST update `top_line` so the cursor's display row is within `[min_row, max_row]`.

### Horizontal follow (no-wrap only)

When `wrap = false`:

1. Compute cursor display column `c_col` using actual grapheme display widths.
2. `c_x = c_col - left_col`
3. `min_x = h_margin`, `max_x = text_cols - 1 - h_margin`
4. If `c_x < min_x`, set `left_col := c_col - min_x`
5. If `c_x > max_x`, set `left_col := c_col - max_x`
6. Clamp `left_col`.

### Explicit positioning commands

| Command | Requirement |
|---|---|
| Center (`zz`) | Cursor display row MUST become `floor(text_rows / 2)`. |
| Top (`zt`) | Cursor display row MUST become `0`. |
| Bottom (`zb`) | Cursor display row MUST become `text_rows - 1`. |

## Clamping and `past_end`

| `past_end` | Requirement |
|---|---|
| `none` | Last visible content row MUST NOT exceed EOF. |
| `scroll` | Virtual empty rows past EOF are allowed; cursor still clamps to valid positions. |

## Resize handling

When window geometry changes:

- `text_rows` and `text_cols` MUST be recomputed.
- `top_line` and `left_col` MUST be re-clamped.
- Display rows MUST be recomputed for wrapped lines.
- Cursor-follow MUST be re-applied so the cursor remains visible.

## Related

- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Windows: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Scroll configuration: [/docs/spec/features/ui/scroll-customization.md](/docs/spec/features/ui/scroll-customization.md)
