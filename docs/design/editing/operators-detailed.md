# Operators — Detailed Design

Back: [/docs/design/editing/README.md](/docs/design/editing/README.md)

Implementation details for the operator system.
Normative requirements are defined in `/docs/spec/editing/operators/`.

## Overview

Operators are the core mechanism for text manipulation. Each operator takes a range (defined by a motion or text object) and transforms the text within that range.

## Operator Trait

Each operator implements a trait that receives:

| Input | Description |
|---|---|
| Range | Start and end positions (line/column or byte offset) |
| Motion type | Character-wise, line-wise, or block-wise |
| Register | Target register for the operation |
| Count | Repeat count |

## Range Computation

1. Enter operator-pending mode.
2. Read motion or text object.
3. Compute start and end positions.
4. Adjust for inclusive/exclusive motion type.
5. Pass range to operator.

## Line-wise Adjustment

For line-wise motions, the range is expanded to include full lines (from column 0 to end of line including newline).

## Character-wise Inclusive Adjustment

For inclusive motions, the end position is advanced to include the character at the end position (by one grapheme cluster, accounting for CJK width).

## Block-wise Processing

For block-wise operations, the operator processes each line independently between the column range:

1. Determine column range from the two block corners.
2. For each line in the row range:
   - Compute byte offsets for the column range.
   - Apply operator to that line segment.

CJK characters that partially overlap the column boundary are fully included.

## Operator Result

After execution, the operator returns:

| Field | Description |
|---|---|
| `new_cursor` | Where the cursor should be placed |
| `text_change` | The text modification applied |
| `enter_mode` | Mode to enter after (e.g., Insert for `c`) |
| `register_content` | Text stored in register (for `d`, `y`) |

## Undo Integration

Each operator invocation creates a single undo entry containing all text changes.

## Dot Repeat

The operator + motion/text-object combination is recorded for dot repeat. `d2w` followed by `.` deletes 2 more words.

## Related

- Operator grammar: [/docs/spec/editing/operators/operator-grammar.md](/docs/spec/editing/operators/operator-grammar.md)
- Operator-pending: [/docs/spec/editing/operators/operator-pending.md](/docs/spec/editing/operators/operator-pending.md)
- Motions detailed: [/docs/design/editing/motions-detailed.md](/docs/design/editing/motions-detailed.md)
- Text objects detailed: [/docs/design/editing/text-objects-detailed.md](/docs/design/editing/text-objects-detailed.md)
