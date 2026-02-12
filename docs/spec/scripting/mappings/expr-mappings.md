# Expression Mappings

Dynamic mappings using expressions.

## Overview

Expression mappings evaluate an expression at invocation time and use the result string as the key sequence to execute. This enables context-dependent key behavior.

## Syntax (normative)

The `<expr>` flag in a mapping declaration makes it an expression mapping:

- `:map <expr> {lhs} {expr}` - the expression is evaluated when `{lhs}` is typed, and the result is fed back as input keys.

## Evaluation Context

The expression has access to:

| Function / Variable | Returns |
|---|---|
| `mode()` | Current mode string |
| `pumvisible()` | 1 if popup menu is visible, 0 otherwise |
| `col(".")` | Current column number |
| `line(".")` | Current line number |
| `&filetype` | Current buffer filetype |
| `&modified` | Whether buffer is modified |
| `bufname()` | Current buffer name |
| `winnr()` | Current window number |
| `visualmode()` | Last visual mode character (`v`, `V`, or `Ctrl-v`) |

## Common Patterns

Frequently used expression mapping patterns.

### Smart Tab

Map Tab to trigger completion if the popup menu is visible, otherwise insert a literal Tab:

- `<expr>` mapping for `<Tab>`: if `pumvisible()` returns 1, produce `<C-n>`; otherwise produce `<Tab>`.

### Smart j/k (display line navigation)

Map `j` and `k` to move by display lines when no count is given (useful with `wrap`):

- `<expr>` mapping for `j`: if count is 0, produce `gj`; otherwise produce `j`.
- `<expr>` mapping for `k`: if count is 0, produce `gk`; otherwise produce `k`.

### Conditional by Mode

An expression mapping in visual mode can check `visualmode()` to decide behavior.

## Ternary Expressions

Expressions use ternary syntax: `condition ? true_value : false_value`. These can be nested for multi-way branching.

## Return Value

The expression MUST return a string. The string is interpreted as a sequence of key presses (with special keys like `<CR>`, `<Esc>` recognized). An empty string means "do nothing."

## Related

- Key mappings: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)
- Modes: [/docs/spec/modes/README.md](/docs/spec/modes/README.md)
