# Viewport Management

Back: [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

Deterministic mapping from logical content to visible window cells.

## Scope

- viewport state is core-owned and window-local
- focused cursor/caret remains visible after every state transition
- wrapping behavior is deterministic for editor, explorer, and terminal views

## Viewport State (per window)

| Field | Meaning |
|---|---|
| `top_line` | first logical content line/row in view |
| `left_col` | first display column when `wrap=false` |
| `wrap` | soft-wrap lines to visible width |
| `text_rows` | visible content rows |
| `text_cols` | visible content columns |
| `scrolloff` | vertical focus margin |
| `sidescrolloff` | horizontal focus margin |

## Gutter and Line Numbers (normative)

For buffer windows, each visible rendered row MUST include a gutter cell group
that presents line identity.

| Row Type | Numbering Requirement |
|---|---|
| normal logical line row | show that line's absolute 1-based number |
| wrapped continuation row | show continuation marker with owning line number |
| filler row below EOF | show filler marker (`~`) and no logical line number |

Line number rendering is window-local and must remain deterministic under resize,
wrap toggles, and split churn.

## Long-Line Safety Contract

When `wrap=true`:

- no rendered cell may exceed right boundary
- overflow becomes continuation rows, not off-screen cells
- width-2 graphemes remain atomic across row boundaries

When `wrap=false`, horizontal scrolling (`left_col`) MUST keep all rendered cells
within bounds and MUST NOT silently clip grapheme halves.

## Wrap Algorithm (normative)

For each grapheme in display order:

1. compute display width (`1` or `2`, zero-width marks merge with owner)
2. if width is `2` and only one column remains:
   - emit one padding cell at row end
   - continue grapheme on next row at column `0`
3. emit grapheme head cell and continuation cell (if width `2`)
4. if row fills, advance to next row
5. stop emitting when viewport row budget is exhausted

Continuation cells are non-addressable for cursor placement.

## Cursor/Caret Visibility

| View Type | Focus Target |
|---|---|
| Buffer | grapheme cursor |
| Explorer | selected node row |
| Terminal | terminal cursor |

After any cursor move, wrap toggle, resize, split change, or focus change:

1. clamp viewport offsets to valid ranges
2. recompute wrapped row mapping
3. apply cursor-follow margins
4. assert focused target is visible or clamp deterministically

## Explicit Positioning Commands

| Command | Requirement |
|---|---|
| `zz` | focused display row becomes center |
| `zt` | focused display row becomes top |
| `zb` | focused display row becomes bottom |

## Determinism Invariants

| Rule | Requirement |
|---|---|
| same input, same output | identical snapshot + geometry yields identical visible rows |
| no hidden overflow | no emitted cell outside viewport bounds |
| stable breakpoints | unchanged content/geometry keeps same wrap points |
| focus safety | exactly one focused target per window |

## Mandatory Verification

| ID | Scenario |
|---|---|
| `WRAP-11R` | 10k ASCII line wraps with no overflow |
| `WRAP-12R` | 10k CJK line wraps with no split-wide artifact |
| `WRAP-13R` | wrap->nowrap->wrap preserves deterministic breakpoints |
| `WRAP-14R` | resize storm preserves on-screen guarantees |
| `WRAP-15R` | repeated narrow geometries have deterministic clamping |
| `WRAP-16R` | editor/explorer/terminal all obey viewport bounds |
| `UI-01` | gutter line number exists for each visible buffer row |
| `UI-02R` | wrapped continuation row numbering remains stable during resize churn |

## Related

- Cursor semantics: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Window model: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- Cursor display: [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md)
- Gutter line numbers: [/docs/spec/features/ui/gutter-line-numbers.md](/docs/spec/features/ui/gutter-line-numbers.md)
