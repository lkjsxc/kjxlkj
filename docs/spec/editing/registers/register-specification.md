# Register Specification

Using registers with operators.

## Overview

A register prefix typed before an operator directs
the yank/delete/paste to a specific register instead
of the unnamed register.

## Syntax

The register prefix is `"{char}` typed in normal mode
before an operator. `"{char}` is also accepted in visual
mode before an operator.

### Examples

| Input | Meaning |
|-------|---------|
| `"add` | Delete line into register `a` |
| `"byy` | Yank line into register `b` |
| `"cp` | Paste from register `c` |
| `"+y$` | Yank to end of line into system clipboard |

## Named Registers

### Lowercase (a-z)

`"a` through `"z` — 26 named registers.
Setting a lowercase register replaces its previous content.

### Uppercase (A-Z)

`"A` through `"Z` — same 26 registers used in append mode.
Setting an uppercase register appends to the corresponding
lowercase register.

### Example

`"ayy` yanks line into `a`. Then `"Ayy` on another line
appends that line to `a`. Register `a` now contains both lines.

## Special Registers

### Unnamed Register ("")

All yank and delete operations write to the unnamed register
unless a register is explicitly specified. `p` without a
register prefix pastes from the unnamed register.

### Clipboard Registers

| Register | System Clipboard |
|----------|------------------|
| `"+` | System clipboard (X11 CLIPBOARD, macOS pasteboard) |
| `"*` | Primary selection (X11 PRIMARY, same as `"+` on macOS/Win) |

### Example

`"+yy` yanks the current line to the system clipboard.
`"*p` pastes from the primary selection.

## Read-Only Registers

| Register | Content |
|----------|---------|
| `"%` | Current file path (relative to cwd) |
| `"#` | Alternate file path |
| `":` | Last executed ex command |
| `".` | Last inserted text |
| `<C-r><C-w>` | Word under cursor (insert/cmdline) |

Attempting to write to a read-only register produces
an error message.

## Numbered Registers

### Yank History

`"0` always contains the most recent yank (not delete).
`"1` through `"9` contain the last 9 deletes, with `"1`
being the most recent. Each new delete pushes older
content from `"1`→`"2`→...→`"9` (oldest discarded).

### Behavior

- `"0`: Last yank only (not affected by deletes)
- `"1`–`"9`: Delete history stack (most recent first)
- Small deletes (less than one line) go to `"-` instead

## Expression Register

### Evaluate

`"=` opens an expression prompt. The evaluated result
is used as the register content for the next paste.

### Usage

`"=2+2<CR>p` pastes `4`. The expression is evaluated
as a script expression (see scripting spec).

### In Insert Mode

`<C-r>=` in insert mode evaluates an expression and
inserts the result at the cursor.

## Black Hole Register

### Discard

`"_` is the black hole register. Anything written to it
is discarded. Reading from it returns empty string.

### Example

`"_dd` deletes a line without affecting any other register.
Useful to avoid polluting the unnamed register.

## Small Delete Register

### Small Deletes

`"-` holds the most recent delete that was less than
one line (e.g. `dw`, `x`, `dl`). Does not shift the
numbered registers.

## Search Register

### Last Search

`"/` contains the most recent search pattern.
Setting it changes what `n`/`N` search for.

## With Operators

### Delete to Register

`"ad{motion}` deletes the motion range into register `a`.
The unnamed register is also updated (unless `a` is `_`).

### Yank to Register

`"ay{motion}` yanks the motion range into register `a`.
The unnamed register and `"0` are also updated.

