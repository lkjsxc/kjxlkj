# Operator-Pending Mode

The state between operator and motion.

## Overview

After typing an operator key (`d`, `c`, `y`, `>`, `<`, `=`,
`g~`, `gu`, `gU`, `gq`, `g?`, `!`, `zf`), the editor enters
operator-pending mode. It remains in this mode until a motion
or text object completes the operation, or the operation
is canceled.

## Mode Indication

### Visual Cues

The cursor changes shape to a half-block during
operator-pending mode (configurable via `guicursor`).
The statusline shows the pending operator character.

### State

The mode object stores: the operator key, the count prefix,
the register prefix (if any), and the moment of entry
(for timeout tracking).

## Entering Mode

### Single Key Operators

| Key | Operator |
|-----|----------|
| `d` | Delete |
| `c` | Change |
| `y` | Yank |
| `>` | Indent right |
| `<` | Indent left |
| `=` | Auto-indent |
| `!` | Filter through external command |

### Two-Key Operators

| Keys | Operator |
|------|----------|
| `g~` | Toggle case |
| `gu` | Lowercase |
| `gU` | Uppercase |
| `gq` | Format |
| `gw` | Format (keep cursor) |
| `g?` | ROT13 encode |
| `zf` | Create fold |

### With Count

A count typed before the operator is stored and applied
when the motion is received: `3d` enters operator-pending
with count=3.

### With Register

`"a` typed before the operator sets the target register:
`"ad` enters operator-pending for delete-into-a.

## Completing Operation

### With Motion

Any motion key completes the operation. The operator
acts on the text from the cursor to the motion target.
`dw` — delete to next word. `c$` — change to end of line.

### With Text Object

Text objects (prefixed `i`/`a`) define a region around
the cursor. `diw` — delete inner word. `ca{` — change
around curly braces.

### With Search

`d/pattern<CR>` deletes from cursor to the first match
of the pattern. `d?pattern<CR>` searches backward.

## Canceling

### Commands

| Key | Effect |
|-----|--------|
| `<Esc>` | Cancel, return to normal mode |
| `<C-c>` | Cancel, return to normal mode |
| `<C-[>` | Cancel, return to normal mode |

### Effect

The pending count and register are discarded.
The buffer is unchanged.

## Timeout

### Configuration

`timeoutlen` (default 1000 ms) is the maximum wait time
for a mapped key sequence. This also applies in
operator-pending mode for multi-key mappings.

For single-key motions there is no timeout — the editor
waits indefinitely.

### Behavior

If a mapped sequence times out, the longest unambiguous
prefix is used. If no valid motion results, the operation
is canceled as if `<Esc>` was pressed.

## Double Operator

### Line Operation

Doubling the operator key (`dd`, `cc`, `yy`, `>>`, `<<`,
`==`, `!!`) applies the operation linewise to the current
line (plus count-1 lines below).

### With Count

`3dd` = delete 3 lines. The doubled key exits
operator-pending immediately (no motion needed).

## Custom Mappings

### Operator-Pending Maps

Mappings can be defined specifically for operator-pending
mode using `omap` / `onoremap` (in TOML config).
These define custom motions available only after an operator.

### Text Objects

Custom text objects are implemented as operator-pending
mappings that set the visual selection to define the
operated region. See operator-mappings.md.

## Motion Types

The motion type determines how the operated region is shaped.

### Characterwise

Most horizontal motions (`w`, `e`, `b`, `f`, `t`, `l`, `h`)
are characterwise. The operation covers a contiguous range
of characters.

### Linewise

Vertical motions (`j`, `k`, `G`, `gg`) and doubled operators
(`dd`, `yy`) are linewise. The operation covers complete lines.

### Blockwise

Only entered via `<C-v>` visual mode. Operators in blockwise
visual mode apply to a rectangular region.

## Exclusive vs Inclusive

### Exclusive

The motion target character is not included. Examples:
`w`, `b`, `W`, `B`, `ge`. `dw` does not delete the first
character of the next word.

### Inclusive

The motion target character is included. Examples:
`e`, `f{char}`, `t{char}`, `$`. `de` deletes through the
last character of the current word.

## Count Multiplication

### Before Operator

Counts before the operator and before the motion multiply:
`2d3w` = `d6w`. See count-with-operators.md.

