# Normal Command

Execute normal mode commands from Ex.

## Overview

The `:normal` command executes normal mode commands
programmatically on lines, enabling batch operations.

## Basic Syntax

`:normal {commands}` executes `{commands}` as if typed
in normal mode. The commands string is a sequence of
keystrokes.

## Simple Examples

### Single Command

`:normal dd` deletes the current line.
`:normal 0` moves to column 1.

### Multiple Commands

`:normal 0i// ` inserts `// ` at line start (comments line).
`:normal ^dw` deletes first word after indentation.

## With Range

### Multiple Lines

`:%normal A;` appends `;` to every line in the buffer.
`:10,20normal dd` deletes lines 10-20 (one at a time).

### Pattern Range

`:g/TODO/normal dd` deletes all lines containing `TODO`.

## Bang Modifier (!)

### Ignore Mappings

`:normal! dd` uses built-in `dd` regardless of mappings.
Without `!`, user's custom mappings are applied.

### Recommended

Always use `!` in scripts and macros for predictable
behavior. Custom mappings can cause unexpected results.

## Special Keys

### Escape Sequence

`<Esc>` cannot be typed literally in the command line.
Use `execute` to include special keys.

### With Execute

`:execute "normal! i" . variable . "\<Esc>"` inserts
variable content and escapes. The `\<Esc>` represents
the escape key in the string.

## Common Uses

### Comment Lines

`:%normal! I// ` prepends `// ` to every line.
`:10,20normal! I# ` comments lines 10-20 with `#`.

### Append to Lines

`:%normal! A;` appends semicolons to all lines.
`:g/return/normal! A // checked` annotates return lines.

### Delete Characters

`:%normal! x` deletes first character of every line.
`:%normal! $x` deletes last character of every line.

### Indent

`:%normal! >>` indents all lines by one level.
`:10,20normal! >>` indents lines 10-20.

## With Global

### Process Matching Lines

`:g/pattern/normal! @a` runs macro `a` on matching lines.
`:g/^$/normal! dd` deletes all blank lines.

### Complex Actions

`:g/fn /normal! f(yi(` yanks function parameters on
each line containing `fn `.

## Macros

### Execute Macro

`:%normal! @a` runs macro `a` on every line.

### Record + Apply

1. `qa` — Start recording macro `a`
2. Edit one line as desired
3. `q` — Stop recording
4. `:%normal! @a` — Apply to all lines

## Motion Commands

### Navigate and Act

`:normal! gg` moves to the first line.
`:normal! G` moves to the last line.

### Search

`:normal! /pattern` starts a search (but cannot press
Enter from within `:normal`). Use `:execute` for this.

## Insert Mode

### Enter and Exit

`:normal! ihello\<Esc>` inserts "hello" and exits insert.
The `\<Esc>` must be provided via `:execute`.

### Practical

`:execute "normal! Iprefix: \<Esc>"` inserts prefix text.

## Visual Mode

### Select and Act

`:normal! viwd` selects inner word then deletes.

### Block Operations

`:normal! V>` selects line and indents (same as `>>`).

## Operators

### Delete

`:normal! d$` deletes to end of line.
`:normal! daw` deletes a word with surrounding space.
`:normal! d%` deletes to matching bracket.
