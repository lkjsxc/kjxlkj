# Double Operators

Line-wise shortcuts with repeated operators.

## Overview

Repeating an operator key applies the operation
to the current line (linewise). The doubled key
acts as a shorthand for `operator` + `_` (current line motion).

## Delete Line

### Command

`dd` deletes the current line, including its trailing newline.
The line below moves up to fill the gap.

### Behavior

The deleted text is placed in the unnamed register (`""`) and
into `"1`, shifting `"1`–`"8` to `"2`–`"9`.
The cursor moves to the first non-blank of the new current line.

### With Count

`[count]dd` deletes count lines starting from the current line.

### With Register

`"add` deletes the current line into register `a`.
`"Add` appends the deleted line to register `a`.

## Yank Line

### Command

`yy` yanks the current line into the unnamed register (`""`) and `"0`.
The cursor does not move.

### Alias

`Y` is equivalent to `yy` (Vim default; configurable to `y$`).

### With Count

`[count]yy` yanks count lines.

### With Register

`"ayy` yanks into register `a`; `"Ayy` appends.

## Change Line

### Command

`cc` clears the current line’s content and enters insert mode.
The line itself remains (not deleted).

### Behavior

When `autoindent` is set, the cursor is placed at the
indentation level of the original line. Otherwise column 0.

### Alias

`S` is equivalent to `cc`.

### With Count

`[count]cc` clears count lines, replacing them with a single
blank line in insert mode.

## Indent Line

### Shift Right

`>>` shifts the current line right by `shiftwidth` columns.
Adds spaces or tabs depending on `expandtab`.

### Shift Left

`<<` shifts the current line left by `shiftwidth` columns.
Stops at column 0 (never produces negative indent).

### With Count

`[count]>>` shifts count lines. Dot-repeatable.

## Auto-Indent

### Command

`==` re-indents the current line based on syntax/indentation rules.
Uses the configured `indentexpr` or built-in C-style indent.

### Behavior

Fixes indentation based on surrounding context.
Does not alter non-whitespace content.

### With Count

`[count]==` re-indents count lines.

## Format Line

### Command

`gqq` formats the current line.

### Behavior

Wraps the line at `textwidth`. Joins short neighboring lines.
Respects `formatoptions` flags (comment leaders, list items).

### With Count

`[count]gqq` formats count lines.

## Case Line

### Toggle

`g~~` toggles case of every character on the current line.

### Lowercase

`guu` converts the current line to lowercase.

### Uppercase

`gUU` converts the current line to uppercase.

### With Count

`[count]g~~` etc. operate on count lines.

## Filter Line

### Command

`!!` filters the current line through an external command.
The editor prompts for the command after `!!`.

### Example

`!!sort` replaces the current line with the output of `sort`.
`3!!sort` filters 3 lines through `sort`.

### With Count

`[count]!!` filters count lines.

## Comment Line

### Plugin Command

`gcc` toggles line comment on the current line using the
filetype’s comment string (`commentstring` option).
With count: `[count]gcc` toggles count lines.

