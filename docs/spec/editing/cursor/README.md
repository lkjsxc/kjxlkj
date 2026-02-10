# Cursor Semantics

Back: [/docs/spec/editing/README.md](/docs/spec/editing/README.md)

Cursor behavior is core-owned, deterministic, and grapheme-based.

## Fundamental Model

Cursor position is `(line, grapheme_offset)`.

- cursor MUST always rest on a grapheme boundary
- internal half-grapheme positions are forbidden
- display-column mapping is derived, not authoritative state

## Grapheme and Width Rules

| Term | Definition |
|---|---|
| Grapheme cluster | user-perceived character unit (UAX #29) |
| Grapheme offset | zero-based index in line grapheme sequence |
| Display width | rendered cell width for grapheme (ASCII 1, many CJK 2) |

| Rule | Requirement |
|---|---|
| Width safety | width-2 graphemes remain atomic for motion and rendering |
| No half-cell cursor | cursor never lands on second cell of width-2 grapheme |
| Mixed script safety | ASCII+CJK motion always moves one grapheme at a time |

## Mode-Dependent Valid Range

For line with `G` graphemes:

| Mode Class | Valid Offsets |
|---|---|
| Normal / Visual / Replace | `0..G-1` when `G>0`, otherwise `0` |
| Insert | `0..G` |

Leaving Insert MUST clamp to Normal-valid range.

## Insert/Append Semantics

| Key | Required Behavior |
|---|---|
| `i` | enter Insert at current grapheme offset |
| `a` | enter Insert at `min(current + 1, G)` |
| `A` | move to end-of-line then enter Insert at `G` |

### End-of-line Rule

If cursor is on last grapheme and `a` is pressed:

- insertion offset becomes `G`
- this MUST differ from `i`, which keeps current offset

### Shift Normalization Dependency

`Shift+a` in Normal mode MUST be decoded as `A` before mode dispatch.

## Mapping Functions

| Direction | Function |
|---|---|
| Grapheme -> display | `display_col(grapheme_offset)` |
| Display -> grapheme | `grapheme_at_display_col(col)` |

For continuation cell of width-2 grapheme, `grapheme_at_display_col` MUST return the owning grapheme offset.

## Cursor Display Integration

Cursor rendering must follow [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md) and wrap rules in [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md).

## Mandatory Regression Tests

| ID | Scenario |
|---|---|
| `CUR-01` | `a` at non-EOL inserts after current grapheme |
| `CUR-02` | `a` at EOL inserts after final grapheme |
| `CUR-03` | `i` at EOL differs from `a` at EOL |
| `CUR-04` | `Shift+a` dispatches to `A` before Insert |
| `CUR-05` | repeated `a` + `Esc` never leaves floating insert-range cursor |
| `CUR-06` | mixed ASCII+CJK append keeps grapheme-safe boundaries |
| `CUR-07R` | cursor remains visible across wrap and resize churn |
| `CUR-08R` | width-2 grapheme highlight covers both cells |

## Related

- Mode entry keys: [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- Viewport rules: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Cursor display: [/docs/spec/features/ui/cursor-customization.md](/docs/spec/features/ui/cursor-customization.md)
