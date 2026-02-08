# Operators
Operators transform text over a range defined by a motion or text object.

## Requirements
- Operators are deterministic, core-owned operations.
- Operator + motion MUST resolve to an explicit range before mutation.
- Composite actions commit as a single transaction (one undo unit).
- Multi-cursor applies the same operator deterministically across cursors.

## Core operators (normative)

| Key | Operator | Description | Register behavior |
|---|---|---|---|
| `d` | Delete | Remove text in range; text goes to `"` and rotates if linewise. | Writes `"`, rotates `1`–`9` if ≥1 line, else writes `-`. |
| `y` | Yank | Copy text in range without modifying buffer. | Writes `"` and `0`. |
| `c` | Change | Delete range and enter Insert mode. | Same as `d`. |
| `>` | Indent | Increase indent by `shiftwidth` spaces. | No register update. |
| `<` | Outdent | Decrease indent by `shiftwidth` spaces. | No register update. |
| `=` | Auto-indent | Re-indent lines to match surrounding context. | No register update. |
| `gu` | Lowercase | Convert text to lowercase. | No register update. |
| `gU` | Uppercase | Convert text to uppercase. | No register update. |
| `g~` | Toggle case | Swap case of each character. | No register update. |
| `gq` | Format | Reflow text to `textwidth`. | No register update. |
| `!` | Filter | Pipe range through external command. | No register update. |

## Line-doubled forms (normative)

| Key sequence | Equivalent | Description |
|---|---|---|
| `dd` | `d` + current line | Delete current line |
| `yy` | `y` + current line | Yank current line |
| `cc` | `c` + current line | Change current line |
| `>>` | `>` + current line | Indent current line |
| `<<` | `<` + current line | Outdent current line |
| `==` | `=` + current line | Auto-indent current line |
| `guu` / `gugu` | `gu` + current line | Lowercase current line |
| `gUU` / `gUgU` | `gU` + current line | Uppercase current line |
| `g~~` / `g~g~` | `g~` + current line | Toggle case on current line |
| `gqq` / `gqgq` | `gq` + current line | Format current line |

## Special forms (normative)

| Key | Description | Register behavior |
|---|---|---|
| `x` | Delete character under cursor (like `dl`) | Writes `"`, writes `-`. |
| `X` | Delete character before cursor (like `dh`) | Writes `"`, writes `-`. |
| `r{c}` | Replace character under cursor with `{c}` | No register update. |
| `s` | Substitute character (like `cl`) | Same as `c`. |
| `S` | Substitute line (like `cc`) | Same as `c`. |
| `C` | Change to end of line (like `c$`) | Same as `c`. |
| `D` | Delete to end of line (like `d$`) | Same as `d`. |
| `Y` | Yank current line (like `yy`) | Same as `y`. |
| `J` | Join current line with next (space-separated) | No register update. |
| `gJ` | Join without adding space | No register update. |
| `~` | Toggle case of character under cursor and advance | No register update. |

## Operator + range resolution (normative)

1. User types operator key (e.g., `d`). Editor enters OperatorPending mode.
2. User types motion or text-object (e.g., `w`, `iw`, `3j`).
3. The motion/text-object resolver computes a `(start, end, type)` range where type is `Characterwise`, `Linewise`, or `Blockwise`.
4. The operator applies to the resolved range.
5. If the range type is `Linewise`, the operator affects entire lines regardless of cursor column.
6. If the range type is `Characterwise` and the motion is inclusive, the end position is included; if exclusive, it is not.

## Count interaction (normative)

Both operator and motion may have counts. They multiply: `2d3w` deletes 6 words.

## Visual mode interaction (normative)

In Visual mode, the operator is applied to the selected region. The selection type (char/line/block) determines the range type. After the operator executes, the editor returns to Normal mode.

## Dot repeat (normative)

The last change (operator + motion/text-object + inserted text if any) is recorded. The `.` command replays it at the current cursor position with optional new count.

## Related

- Text objects: [/docs/spec/editing/text-objects/text_objects.md](/docs/spec/editing/text-objects/text_objects.md)
- Registers: [/docs/spec/editing/registers/registers.md](/docs/spec/editing/registers/registers.md)
- Undo: [/docs/spec/editing/text-manipulation/undo.md](/docs/spec/editing/text-manipulation/undo.md)
- Advanced (dot repeat, macros): [/docs/spec/editing/operators/advanced.md](/docs/spec/editing/operators/advanced.md)
- Input decoding (operator-pending): [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
