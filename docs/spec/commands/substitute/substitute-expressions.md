# Substitute Expressions

Dynamic replacements with `\=`.

## Overview

The `\=` replacement prefix in `:s` commands evaluates an expression and uses the result as replacement text, enabling computed substitutions.

## Syntax (normative)

`:[range]s/pattern/\={expression}/[flags]`

The expression is evaluated for each match. The returned string replaces the matched text.

## submatch() Function (normative)

| Call | Returns |
|---|---|
| `submatch(0)` | The entire matched text |
| `submatch(1)` | First captured group `\(...\)` |
| `submatch(2)` | Second captured group |
| `submatch(N)` | Nth captured group |

## Common Expression Patterns

| Goal | Expression |
|---|---|
| Uppercase match | `\=toupper(submatch(0))` |
| Lowercase match | `\=tolower(submatch(0))` |
| Increment numbers | `\=submatch(0) + 1` |
| Multiply by 10 | `\=submatch(1) * 10` |
| String length | `\=strlen(submatch(0))` |
| Line number | `\=line(".")` |
| Date stamp | `\=strftime("%Y-%m-%d")` |
| Sequential counter | `\=line(".") - line("'<") + 1` (within visual selection) |
| Conditional | `\=submatch(1) == "yes" ? "no" : "yes"` |
| Pad with zeros | `\=printf("%04d", submatch(0))` |
| Register content | `\=@a` |
| Concatenation | `\=submatch(1) . "-" . submatch(2)` |

## Evaluation Context

The expression runs in the context of the buffer being edited. `line(".")` returns the line number of the current match. Variables and functions from the scripting environment are accessible.

## Error Handling

If the expression evaluation fails, the substitution is aborted and an error message is displayed. The buffer is left in the state prior to the failed match.

## Related

- Substitute command: [/docs/spec/commands/substitute/README.md](/docs/spec/commands/substitute/README.md)
- Substitute specials: [/docs/spec/commands/substitute/substitute-specials.md](/docs/spec/commands/substitute/substitute-specials.md)
- Expression register: [/docs/spec/editing/registers/expression-register.md](/docs/spec/editing/registers/expression-register.md)
