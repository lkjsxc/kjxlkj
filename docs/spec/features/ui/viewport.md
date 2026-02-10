# Viewport Management

Back: [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

Deterministic rules for mapping content to visible window cells.

## Scope

- viewport state is core-owned and window-local
- focused cursor/caret must remain visible after any state transition
- wrapping and overflow behavior must be deterministic for editor, explorer, and terminal views

## Viewport State (per window)

| Field | Type | Meaning |
|---|---|---|
| `top_line` | integer | first logical content line/row in view |
| `left_col` | integer | first display column (no-wrap mode only) |
| `wrap` | boolean | soft-wrap lines to view width |
| `text_rows` | integer | visible content rows |
| `text_cols` | integer | visible content columns |
| `scrolloff` | integer | vertical cursor margin |
| `sidescrolloff` | integer | horizontal cursor margin |

## Defaults

| Field | Default | Rule |
|---|---|---|
| `wrap` | `true` | if `true`, `left_col` MUST be `0` |
| `top_line` | `0` | clamped by content extent |
| `left_col` | `0` | used only when `wrap=false` |

## Long-Line Safety Contract

When `wrap=true`:

- rendered content MUST NOT exceed right boundary of text area
- overflow MUST produce continuation rows, never off-screen cells
- wide graphemes MUST remain atomic across row boundaries

## Wide Grapheme Boundary Rule

If only one column remains and next grapheme has width 2:

1. current row receives one padding cell
2. grapheme is rendered at start of next row
3. no half-cell cursor position may exist

## Cursor/Caret Visibility Rules

| View Type | Focus Target |
|---|---|
| Buffer | grapheme cursor |
| Explorer | selected node row |
| Terminal | terminal cursor |

After cursor/caret change, resize, wrap toggle, or focus change, viewport must be re-clamped and visibility re-evaluated.

## Cursor-Follow (normative)

For focused window:

1. compute effective vertical/horizontal margins from `scrolloff` and `sidescrolloff`
2. if focused row is above top margin, decrease `top_line`
3. if focused row is below bottom margin, increase `top_line`
4. when `wrap=false`, apply equivalent horizontal follow for `left_col`
5. clamp resulting offsets to valid ranges

## Explicit Positioning Commands

| Command | Requirement |
|---|---|
| `zz` | cursor display row becomes center |
| `zt` | cursor display row becomes top row |
| `zb` | cursor display row becomes bottom row |

## Resize Handling

On geometry change:

- recompute `text_rows` and `text_cols`
- recompute wrapped display rows where needed
- clamp viewport offsets
- rerun cursor-follow to maintain visibility

## Determinism Invariants

| Rule | Requirement |
|---|---|
| Same input, same output | identical snapshot + geometry yields identical visible rows |
| No hidden overflow | no emitted cell outside viewport bounds |
| Stable breakpoints | without content/geometry change, wrap points remain identical |
| Focus safety | exactly one focused caret target per window |

## Mandatory Verification

| ID | Scenario |
|---|---|
| `WRAP-11R` | 10k ASCII line wraps without overflow |
| `WRAP-12R` | 10k CJK line wraps without split wide grapheme |
| `WRAP-13R` | wrap->nowrap->wrap toggling preserves deterministic breakpoints |
| `WRAP-14R` | resize storm preserves on-screen guarantees |
| `WRAP-15R` | 1x1 geometry clamp has no panic and deterministic output |
| `WRAP-16R` | mixed editor/explorer/terminal windows all obey bounds |

## Related

- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Cursor display: [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md)
