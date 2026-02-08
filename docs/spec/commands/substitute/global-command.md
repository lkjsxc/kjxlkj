# Global Command

Executing commands on matching lines.

## Overview

The `:global` command executes
Ex commands on all lines
matching a pattern.

## Basic Syntax

```
:[range]g/pattern/command
```

Without a range, operates on the entire file (`%` is the default range for `:g`).

## Simple Examples

### Delete Matching

| Command | Effect |
|---------|--------|
| `:g/TODO/d` | Delete all lines containing `TODO` |
| `:g/^$/d` | Delete all empty lines |
| `:g/^#/d` | Delete all lines starting with `#` |

### Print Matching

| Command | Effect |
|---------|--------|
| `:g/ERROR/p` | Print all lines containing `ERROR` |
| `:g/pattern/number` | Print matching lines with line numbers |

## Pattern Delimiters

### Slash Default

The default delimiter is `/`. The command form is `:g/pattern/command`.

### Alternatives

Any single-byte character can replace `/` as the delimiter:

| Example | Delimiter |
|---------|-----------|
| `:g#pattern#d` | `#` |
| `:g+TODO+d` | `+` |

## Common Commands

### Delete

`:g/pattern/d` deletes every line matching the pattern. This is the most common use of `:g`.

### Normal Mode

`:g/pattern/normal dd` executes normal-mode keystrokes on each matching line. The `normal` command runs as if typed in normal mode.

### Substitute

Replace within matching lines.

| Command | Effect |
|---------|--------|
| `:g/TODO/s/old/new/g` | On lines with `TODO`, replace `old` with `new` |
| `:g/^func/s/err/error/g` | On function lines, replace `err` with `error` |

### Move/Copy

| Command | Effect |
|---------|--------|
| `:g/pattern/m$` | Move matching lines to end of file |
| `:g/pattern/t0` | Copy matching lines to top of file |
| `:g/^>/m$` | Move quoted lines to end |

### Execute

`:g/pattern/execute "normal! ..."` evaluates an expression and runs the result as an Ex command on each matching line.

## Range with Global

### Limit Scope

| Command | Effect |
|---------|--------|
| `:1,50g/TODO/d` | Delete `TODO` lines in first 50 lines only |
| `:'<,'>g/pattern/d` | Delete matches within visual selection |
| `:.,.+10g/old/s//new/g` | Substitute in next 10 lines matching `old` |

## Inverse Global

### Vglobal (:v)

Execute on non-matching lines:

`:v/pattern/d` deletes all lines NOT matching the pattern, keeping only matches. See `/home/lkjsxc/repos/kjxlkj/docs/spec/commands/substitute/vglobal-command.md` for full details.

### Equivalent

`:g!/pattern/command` is identical to `:v/pattern/command`. Both execute the command on every line that does NOT match the pattern.

## Complex Examples

### Multiple Conditions

Use regex alternation to match several terms: `:g/TODO\|FIXME\|HACK/d` deletes lines matching any of the three words.

### Nested Patterns

Substitute within function bodies.

`:g/^function/,/^}/s/var/let/g` uses a range after `:g` to replace `var` with `let` inside each function block.

### With Marks

`:g/pattern/mark a` sets mark `a` on matching lines. Only the last match retains the mark since each execution overwrites it.

## Chained Commands

### Bar Separator

Use `|` to run multiple Ex commands per match: `:g/pattern/s/a/b/ | s/c/d/`. The `|` separates commands that each execute on the matching line.

### Multiple Substitutes

`:g/pattern/s/a/b/g | s/c/d/g` runs two substitutions sequentially on each line matching `pattern`.

## Normal Command

### Single Key

| Command | Effect |
|---------|--------|
| `:g/pattern/normal dd` | Delete each matching line |
| `:g/pattern/normal >>` | Indent each matching line |
| `:g/pattern/normal J` | Join each matching line with the next |

### Multiple Keys

Add "# " at start.

`:g/pattern/normal I# ` inserts `# ` at the beginning of each matching line using normal-mode `I`.

### Macros

`:g/pattern/normal @q` replays macro `q` on each matching line. Record the macro first with `qq...q`.

## Global Counter

### Number Lines

`:let c=0 | g/pattern/let c+=1` counts lines matching the pattern, storing the total in variable `c`.

### Enumerate

`:let c=0 | g/^/let c+=1 | s/^/\=c . ". "/` prefixes every line with a sequential number followed by a period.

## Delete Duplicates

### Keep First

After sorting with `:sort`, use `:g/^\(.*\)\n\1$/d` to remove consecutive duplicate lines, keeping the first occurrence.

### Sort First

Run `:sort` to group identical lines, then `:g/^\(.*\)\n\1$/d` to remove all duplicates from the file.
