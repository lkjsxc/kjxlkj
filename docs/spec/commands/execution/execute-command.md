# Execute Command

Back: [/docs/spec/commands/execution/README.md](/docs/spec/commands/execution/README.md)

Evaluate an expression and execute the result as an ex command.

## Overview

`:execute {expr}` evaluates `{expr}` and runs the resulting string as an ex command. This is primarily used for constructing commands with special keys or variable values.

## Syntax

`:execute {expr} [expr] ...`

Multiple expressions are concatenated with a space.

## Use Cases

Typical scenarios for the execute command.

### Special Keys

`:execute "normal! \<C-a>"` — the `\<C-a>` is evaluated to the actual key code.

### Dynamic Commands

`:execute "edit " . filename` — opens a file stored in a variable.

### With Variables

`:execute line_number . "d"` — deletes the line at the stored number.

## Concatenation

`:execute "normal!" "dd"` — evaluates to `:normal! dd`. Expressions are joined with spaces.

## String Escaping

Strings use double quotes. Backslash sequences are interpreted:

| Sequence | Meaning |
|---|---|
| `\<CR>` | Carriage return |
| `\<Esc>` | Escape |
| `\<C-a>` | Ctrl-A |
| `\\` | Literal backslash |

## Error Handling

If the expression evaluates to an invalid command, an error message is shown. Execution stops at the first error in a chained execute.

## Related

- Execution: [/docs/spec/commands/execution/README.md](/docs/spec/commands/execution/README.md)
- Normal command: [/docs/spec/commands/execution/normal-command.md](/docs/spec/commands/execution/normal-command.md)
- Scripting: [/docs/spec/scripting/README.md](/docs/spec/scripting/README.md)
