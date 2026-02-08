# Expression Register

The `=` register for evaluating expressions and inserting results.

## Overview

The expression register (`=`) prompts the user to enter an expression, evaluates it, and uses the result as the register's content. It enables computed insertions and dynamic behavior.

## Access Points (normative)

| Context | Key sequence | Behavior |
|---|---|---|
| Insert mode | `Ctrl-r =` | Prompts for expression, inserts result at cursor |
| Normal mode | `"=` then operator | Expression result used as register content |
| Command line | `Ctrl-r =` | Inserts expression result into command line |

## Expression Syntax (normative)

| Category | Examples | Result |
|---|---|---|
| Arithmetic | `2+3`, `10-4`, `3*4`, `15/4`, `17%5` | Integer math |
| Float | `15.0/4` | `3.75` |
| Strings | `"hello"`, `"a" . "b"` (concatenation) | String values |
| String functions | `strlen("text")`, `toupper("hi")`, `tolower("HI")` | Transformed strings |
| Substitution | `substitute("foo", "o", "a", "g")` | `"faa"` |
| Register access | `@a`, `@"`, `@0`, `@/`, `@%` | Register contents |
| Ternary | `condition ? val_true : val_false` | Conditional value |

## Commonly Used Functions

| Function | Returns |
|---|---|
| `line(".")` | Current line number |
| `col(".")` | Current column number |
| `expand("%")` | Current file name |
| `strftime("%Y-%m-%d")` | Formatted date/time |
| `$HOME` | Environment variable |
| `&option` | Value of a Vim option |

## Expression History

Previous expressions are navigable with Up/Down arrows in the expression prompt. They share the expression history type.

## Error Handling

If the expression is invalid or produces an error, an error message is displayed and no text is inserted. The expression register returns an empty string.

## Related

- Registers: [/docs/spec/editing/registers/README.md](/docs/spec/editing/registers/README.md)
- Insert register access: [/docs/spec/modes/insert/input/insert-registers.md](/docs/spec/modes/insert/input/insert-registers.md)
