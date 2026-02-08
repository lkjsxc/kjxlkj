# Cursor Semantics

Back: [/docs/spec/editing/README.md](/docs/spec/editing/README.md)

Cursor behavior is core-owned, deterministic, and defined over buffer snapshots.

## Fundamental model

The cursor is a logical position `(line, grapheme_offset)` where `grapheme_offset` is the zero-based index into the line's grapheme cluster sequence.

The cursor MUST always rest on a grapheme boundary. There is no concept of a cursor positioned "inside" or on the "back half" of a grapheme cluster.

## Grapheme cluster model (normative)

The text model MUST decompose each line into an ordered sequence of grapheme clusters (Unicode UAX #29). All cursor arithmetic operates on grapheme indices, never on byte offsets or code point indices directly.

| Term | Definition |
|---|---|
| Grapheme cluster | The smallest user-perceived character unit (UAX #29). |
| Grapheme index | Zero-based position in the line's grapheme sequence. |
| Display width | The number of terminal columns a grapheme occupies (1 for most Latin/ASCII, 2 for CJK/fullwidth, 0 for combining marks attached to a base). |

## Column model

This project uses a mode-dependent column model defined over grapheme indices.

### End-exclusive cursor (Normal, Visual, VisualLine, Replace)

For a line with `G` grapheme clusters (`G > 0`): valid grapheme offsets are `0..G-1`.

For an empty line (`G = 0`): only offset `0` is valid.

### End-inclusive cursor (Insert)

The cursor represents an insertion point between graphemes. For a line with `G` grapheme clusters: valid offsets are `0..G` (cursor may be placed after the last grapheme).

## Wide character (CJK) cursor rules (normative)

Wide characters (display width 2) occupy two terminal columns but are a single grapheme cluster at a single grapheme index. The cursor MUST obey:

| Rule | Requirement |
|---|---|
| No half-cell cursor | The cursor MUST always be positioned at the start column of a grapheme cluster, never at the second column of a wide character. |
| Motion atomicity | `h` and `l` motions MUST move one grapheme cluster at a time. On a line of CJK text, `l` advances the display position by 2 columns because each grapheme is width-2. |
| Cursor rendering | When the cursor is on a width-2 grapheme, the block cursor MUST span 2 terminal columns. |
| No phantom states | There MUST NOT exist any internal state where the cursor references a position between the two display columns of a single wide character. |

### Example: CJK cursor motion

Given line content: `あいうえお` (5 grapheme clusters, each width 2, display columns 0-9):

| Grapheme index | Character | Display columns |
|---|---|---|
| 0 | あ | 0-1 |
| 1 | い | 2-3 |
| 2 | う | 4-5 |
| 3 | え | 6-7 |
| 4 | お | 8-9 |

`l` from grapheme index 0 moves to grapheme index 1 (display column 2). There is no state where the cursor is at display column 1.

### Mixed-width lines

For a line containing mixed ASCII and CJK characters (e.g., `aあbいc`):

| Grapheme index | Character | Display columns |
|---|---|---|
| 0 | a | 0 |
| 1 | あ | 1-2 |
| 2 | b | 3 |
| 3 | い | 4-5 |
| 4 | c | 6 |

`l` from index 0 moves to index 1. `l` from index 1 moves to index 2. The cursor always lands on a grapheme boundary.

## Display column mapping (normative)

The implementation MUST maintain a bidirectional mapping between grapheme indices and display columns:

| Direction | Function |
|---|---|
| Grapheme to display | `display_col(grapheme_idx)` returns the starting display column. |
| Display to grapheme | `grapheme_at_display_col(col)` returns the grapheme index whose display range contains `col`. |

For the second column of a wide character, `grapheme_at_display_col` MUST return the same grapheme index as for the first column.

This mapping MUST be recomputed when buffer content changes on the line or tab width changes.

## Append semantics (`a`)

In Normal mode, `a` enters Insert mode with the insertion point after the grapheme under the cursor.

Given a line with `G` grapheme clusters and current cursor at grapheme index `c`:

- The insertion grapheme offset becomes `min(c + 1, G)`.
- Mode becomes Insert.

### Repeated `a` then `Esc` regression guard (normative)

After repeated `a` and `Esc`, the Normal-mode cursor MUST clamp to the last grapheme of the line (or `0` on empty line). The cursor MUST NOT remain at a floating end-inclusive offset in Normal mode.

| Line graphemes | Expected Normal-mode offset after `a ... Esc` |
|---|---|
| `G = 0` | `0` |
| `G > 0` | `G - 1` |

## Mode transition clamping

When transitioning into an end-exclusive mode (Normal/Visual/Replace), the cursor MUST be clamped into the end-exclusive range for the active buffer/line.

This clamping rule is mandatory even after rapid mode churn.

## Tab character handling

Tab characters are single graphemes with variable display width. The cursor model treats each tab as one grapheme at one grapheme index, regardless of how many display columns it occupies.

## Related

- Design rationale: [/docs/design/editing/README.md](/docs/design/editing/README.md)
- Viewport management: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- Unicode guidance: [/docs/technical/unicode.md](/docs/technical/unicode.md)
