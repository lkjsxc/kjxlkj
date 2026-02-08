# Execute Command

Evaluate expressions and execute resulting strings.

## Overview

`:execute` evaluates its arguments as expressions and
executes the resulting string as an Ex command. This
enables dynamic command construction.

## Basic Syntax

`:execute {expr}` evaluates `{expr}` and runs the
result as a command.

## String Concatenation

### Dot Operator

`:execute "normal! " . "dd"` concatenates strings.
Multiple arguments are joined with spaces.

### Variables

`:execute "edit " . filename` uses variable values
to construct commands dynamically.

## Special Key Notation

### Inserting Special Keys

`:execute "normal! i" . text . "\<Esc>"` — the `\<Esc>`
represents the escape key within the string.

### Key Codes

| Notation | Key |
|----------|-----|
| `"\<Esc>"` | Escape |
| `"\<CR>"` | Enter |
| `"\<Tab>"` | Tab |
| `"\<C-w>"` | Ctrl-W |
| `"\<BS>"` | Backspace |

## Common Patterns

### Dynamic Commands

`:execute ":" . line_number` jumps to a computed line.
`:execute "edit " . expand("%:r") . ".test.rs"` opens
the test file for the current source file.

### With Normal

`:execute "normal! " . count . "j"` moves down a
computed number of lines.

### Pattern with Variables

`:execute "g/" . pattern . "/d"` deletes lines matching
a pattern stored in a variable.

## Multiple Commands

### Bar Separator

`:execute "cmd1" | execute "cmd2"` runs two commands.
The `|` separates independent commands.

### Within String

`:execute "cmd1 | cmd2"` — the bar inside the string
is part of the executed command, not a separator.

## Error Handling

### Invalid Command

If the resulting string is not a valid command, an
error message is displayed. Execution of subsequent
commands continues.

### Empty String

`:execute ""` does nothing (no-op).

## Expression Types

### String Expressions

`:execute "echo 'hello'"` runs `echo 'hello'`.

### Numeric Expressions

`:execute 42` is equivalent to `:42` (go to line 42).
Numbers are converted to strings.

### Conditional

`:execute condition ? "cmd1" : "cmd2"` conditionally
executes one of two commands.

## Nesting

### Execute within Execute

`:execute "execute 'normal! dd'"` — execute can be
nested but this is rarely needed.

## Integration

### With Ranges

`:execute line1 . "," . line2 . "d"` deletes a
computed range of lines.

### With Registers

`:execute "normal! \"" . reg . "p"` pastes from
a register stored in a variable.

### With Functions

`:execute "call " . funcname . "()"` calls a
function by computed name.
