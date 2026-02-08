# Count with Operators

Using numeric counts with operators.

## Overview

Counts multiply the effect of operators and motions.
The count is a decimal integer typed before an operator,
before a motion, or both.

## Basic Syntax

The general form is `[count1] operator [count2] motion`.
When both count1 and count2 are present the effective
count is their product.

### Count Before Operator

| Input | Meaning |
|-------|---------|
| `3dd` | Delete 3 lines |
| `2yy` | Yank 2 lines |
| `5>>` | Indent 5 lines |

### Count Before Motion

| Input | Meaning |
|-------|---------|
| `d3w` | Delete 3 words |
| `c2e` | Change 2 word-ends |
| `y4j` | Yank current + 4 lines down |

### Both

| Input | Effective | Meaning |
|-------|-----------|---------|
| `2d3w` | `d6w` | Delete 6 words |
| `3c2l` | `c6l` | Change 6 chars |

## Count Multiplication

When counts appear in both positions the runtime
multiplies them: `effective = count1 * count2`.

### Rule

The multiplication is pure integer arithmetic.
If either count is absent it defaults to 1.

## Count Positions

Both `3dw` and `d3w` mean "delete 3 words".
The positions are semantically interchangeable
for most operators, but the register prefix `"`
must come before the operator.

## With Line Operators

### Delete Lines

`3dd` deletes the current line plus 2 below (3 total).
All deleted lines go to register `"1`, previous
content shifts to `"2`–`"9`.

### Yank Lines

`5yy` yanks 5 lines into the unnamed register
(and `"0`). Lines are stored as linewise.

### Change Lines

`2cc` clears 2 lines, replaces them with a single
blank line, enters insert mode. Preserves leading
indentation when `autoindent` is set.

## With Word Motions

### Word Operations

`d3w` deletes from cursor through 3 word-boundaries.
`c2w` and `c2e` both change 2 words but `c2w` excludes
trailing whitespace while `c2e` includes the last char.

### WORD Operations

`d2W` operates on whitespace-delimited WORDs.
CJK characters each count as a full WORD boundary.

## With Line Motions

### Down

`d3j` deletes current line and 3 lines below (4 total).
Always linewise.

### Up

`d2k` deletes current line and 2 lines above (3 total).
Always linewise.

## Large Counts

Counts up to `2,147,483,647` (i32::MAX) are accepted.
Larger values are clamped to that maximum.

### Practical Limits

- Motion stops at buffer start/end; excess count is ignored
- The operation is a single undo entry regardless of count
- Memory for very large yanks is bounded by buffer size

## Count with Text Objects

Text objects do not accept a count in `i`/`a` position.
`d2iw` is parsed as `d` with count `2` then `iw`.
This selects the 2nd enclosing `iw` level (nesting).

### Use Repeat Instead

For repeated text-object operations, use `.` (dot repeat)
after the first operation.

## Visual Mode Count

### After Selection

A count typed after entering visual mode extends the
selection by that many motion units: `v3e` selects
through 3 word-ends.

### In Visual

Operators in visual mode ignore preceding count;
they operate on the selection.

## Count Memory

### Repeat with Count

Dot repeat (`.`) remembers the original count.
`3dd` followed by `.` deletes another 3 lines.

### Count Override

Typing a new count before `.` overrides the
remembered count: `5.` after `3dd` deletes 5 lines.

## Zero Count

### Special Case

`0` is never a count—it is the "go to column 0" motion.
Counts must start with `1`–`9`.

### Leading Zero

Input `03w` means motion `0` then `3w`, not count `03`.

## Count and Registers

### Order

The register specification must precede the operator:
`"a3dd` — delete 3 lines into register `a`.
`3"add` is also accepted (count then register then operator).

