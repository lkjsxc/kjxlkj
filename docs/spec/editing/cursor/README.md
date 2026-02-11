# Cursor Semantics

Back: [/docs/spec/editing/README.md](/docs/spec/editing/README.md)

Cursor behavior is core-owned, deterministic, and grapheme-based.

## Fundamental Model

Cursor position is `(line, grapheme_offset)`.

- cursor must always rest on a grapheme boundary
- half-grapheme or continuation-cell positions are forbidden
- display-column mapping is derived from grapheme position

## Grapheme and Width Rules

| Term | Definition |
|---|---|
| Grapheme cluster | user-perceived character unit |
| Grapheme offset | zero-based index in line grapheme sequence |
| Display width | rendered width for grapheme (`1` or `2`) |

| Rule | Requirement |
|---|---|
| width safety | width-2 graphemes are atomic in motion and rendering |
| no continuation target | cursor never lands on width-2 continuation cell |
| mixed-script safety | motion steps by grapheme, not by byte |

## Mode-Dependent Valid Range

For line with `G` graphemes:

| Mode Class | Valid Offsets |
|---|---|
| Normal / Visual / Replace | `0..G-1` when `G>0`, otherwise `0` |
| Insert | `0..G` |

Leaving Insert must clamp to Normal-valid range.

## Insert/Append Semantics

| Key | Required Behavior |
|---|---|
| `i` | enter Insert at current grapheme offset |
| `a` | enter Insert at `min(current + 1, G)` |
| `A` | move to end-of-line and enter Insert at `G` |

### End-of-Line Rule

If cursor is on last grapheme and `a` is pressed, insertion offset becomes `G`.
`i` at EOL keeps current offset and must differ observably.

### Shift Dependency

`Shift+a` in Normal mode must normalize to `A` before mode dispatch.

## Mapping Functions

| Direction | Function |
|---|---|
| Grapheme -> display | `display_col(grapheme_offset)` |
| Display -> grapheme | `grapheme_at_display_col(col)` |

For continuation cell columns, `grapheme_at_display_col` returns owner grapheme offset.

## Cursor Display Integration

Cursor rendering follows:

- [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md)
- [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

Exactly one primary cursor is visible in focused context.

## Mandatory Regression Tests

| ID | Scenario |
|---|---|
| `CUR-01` | `a` at non-EOL inserts after current grapheme |
| `CUR-02` | `a` at EOL inserts after final grapheme |
| `CUR-03` | `i` at EOL differs from `a` at EOL |
| `CUR-04` | `Shift+a` dispatches to `A` before Insert |
| `CUR-05` | repeated `a` + `Esc` never leaves invalid offset |
| `CUR-06` | mixed ASCII+CJK append remains grapheme-safe |
| `CUR-07R` | cursor remains visible across wrap/resize churn |
| `CUR-08R` | width-2 grapheme highlight covers both cells |
| `CUR-09R` | cursor never targets continuation cell |
| `CUR-10R` | wrap-boundary cursor has no split artifact |
| `CUR-11R` | rapid focus changes preserve one primary cursor |

## Related

- Mode entry keys: [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- Input decoding: [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- Viewport rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
