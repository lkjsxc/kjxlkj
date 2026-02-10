# Cursor Semantics

Back: [/docs/spec/editing/README.md](/docs/spec/editing/README.md)

Cursor behavior is core-owned, deterministic, and defined over grapheme clusters.

## Fundamental Model

The cursor is a logical position `(line, grapheme_offset)`.

The cursor MUST always rest on a grapheme boundary. Internal states that point
inside a grapheme are forbidden.

## Grapheme and Width Rules

| Term | Definition |
|---|---|
| Grapheme cluster | User-perceived character unit (UAX #29) |
| Grapheme offset | Zero-based index in the line's grapheme sequence |
| Display width | Cell width of a grapheme (ASCII 1, many CJK 2) |

| Rule | Requirement |
|---|---|
| Width safety | Width-2 graphemes remain atomic for motion and rendering |
| No half-cell cursor | Cursor never lands on second cell of width-2 grapheme |
| Mixed script safety | Motions over ASCII+CJK always move one grapheme at a time |

## Mode-Dependent Cursor Range

| Mode Class | Valid Offsets for line with `G` graphemes |
|---|---|
| End-exclusive (Normal/Visual/Replace) | `0..G-1` when `G > 0`, otherwise `0` |
| End-inclusive (Insert) | `0..G` |

When leaving Insert mode, cursor MUST clamp to end-exclusive range.

## Append and Insert Semantics

| Key | Required Behavior |
|---|---|
| `i` | Enter Insert at current grapheme offset |
| `a` | Enter Insert at `min(current + 1, G)` |
| `A` | Move to end-of-line grapheme `G-1`, then enter Insert at `G` |

### End-of-line append rule

If cursor is already on the last grapheme and `a` is pressed:

- Insert position MUST become `G` (after the last grapheme)
- this MUST differ from `i`, which stays at current offset

### Shift normalization dependency

`Shift+a` in Normal mode MUST dispatch as `A`, not as literal `a`.

## Wide Character Examples

For `あいう`:

| Grapheme Offset | Display Columns |
|---|---|
| 0 | 0-1 |
| 1 | 2-3 |
| 2 | 4-5 |

`l` from offset 0 -> offset 1. No state at display column 1 is allowed.

## Mapping Functions

| Direction | Function |
|---|---|
| Grapheme -> display | `display_col(grapheme_offset)` |
| Display -> grapheme | `grapheme_at_display_col(col)` |

For a continuation cell of a width-2 grapheme, `grapheme_at_display_col` MUST
return the owning grapheme offset.

## Mandatory Regression Tests

| ID | Scenario |
|---|---|
| CUR-01 | `a` at non-EOL inserts after cursor grapheme |
| CUR-02 | `a` at EOL inserts after final grapheme |
| CUR-03 | `i` at EOL differs from `a` at EOL |
| CUR-04 | `Shift+a` dispatches to `A` and moves to line end before Insert |
| CUR-05 | Repeated `a` and `Esc` never leaves floating end-inclusive cursor in Normal mode |
| CUR-06 | Mixed ASCII+CJK append keeps cursor on grapheme boundaries |

## Related

- Keybinding mode entry: [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- Viewport management: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Unicode guidance: [/docs/technical/unicode.md](/docs/technical/unicode.md)
