# Vim-Style Expressions

Expression evaluation in commands, mappings, and registers.

## Overview

The expression engine evaluates arithmetic, string, comparison, and logical expressions. Expressions appear in the expression register (`<C-r>=`), substitution replacement (`\=`), expression mappings, and the command line.

## Expression Register

In insert mode or command line, `<C-r>=` opens the expression prompt. Type an expression and press `<Enter>` to insert the result.

Examples:

- `<C-r>=2+3` inserts `5`
- `<C-r>=strftime("%Y-%m-%d")` inserts the current date
- `<C-r>=line(".")` inserts the current line number

## In Substitution

The replacement string can be an expression when prefixed with `\=`:

`:%s/pattern/\=expression/g`

The expression is evaluated for each match. Inside the expression, `submatch(0)` returns the full match and `submatch(1)` through `submatch(9)` return capture groups.

## In Mappings

Expression mappings use `<expr>` flag. The mapping evaluates the expression and uses the result as keystrokes. See [/docs/spec/scripting/mappings/expr-mappings.md](/docs/spec/scripting/mappings/expr-mappings.md).

## Operators (normative)

Arithmetic and comparison operators in expressions.

### Arithmetic

| Operator | Description |
|---|---|
| `+` | Addition |
| `-` | Subtraction / unary negation |
| `*` | Multiplication |
| `/` | Integer division |
| `%` | Modulo |

### String

| Operator | Description |
|---|---|
| `.` | Concatenation |
| `=~` | Regex match (returns index or -1) |
| `!~` | Regex not match |

### Comparison

| Operator | Case | Description |
|---|---|---|
| `==` | `'ignorecase'` dependent | Equal |
| `!=` | `'ignorecase'` dependent | Not equal |
| `==#` | Case-sensitive | Equal |
| `==?` | Case-insensitive | Equal |
| `<` | Dependent | Less than |
| `>` | Dependent | Greater than |
| `<=` | Dependent | Less or equal |
| `>=` | Dependent | Greater or equal |

String comparisons with `#` suffix are always case-sensitive. With `?` suffix always case-insensitive.

### Logical

| Operator | Description |
|---|---|
| `&&` | Logical AND (short-circuit) |
| `\|\|` | Logical OR (short-circuit) |
| `!` | Logical NOT |

### Ternary

`condition ? true_expr : false_expr` — evaluates and returns one branch.

## Functions (normative)

Built-in functions callable from expressions.

### String Functions

| Function | Returns | Description |
|---|---|---|
| `strlen(s)` | `Number` | Byte length of string |
| `strpart(s,start,len)` | `String` | Substring from byte offset |
| `substitute(s,pat,rep,flags)` | `String` | Replace in string |
| `toupper(s)` | `String` | Uppercase |
| `tolower(s)` | `String` | Lowercase |
| `trim(s)` | `String` | Trim whitespace |
| `stridx(s,needle)` | `Number` | First index of needle (-1 if not found) |
| `strridx(s,needle)` | `Number` | Last index of needle |

### List Functions

| Function | Returns | Description |
|---|---|---|
| `len(list)` | `Number` | List length |
| `get(list,idx,default)` | `Any` | Get element with default |
| `add(list,item)` | `List` | Append in-place |
| `remove(list,idx)` | `Any` | Remove and return |
| `sort(list)` | `List` | Sort in-place |
| `reverse(list)` | `List` | Reverse in-place |
| `map(list,expr)` | `List` | Transform each element |
| `filter(list,expr)` | `List` | Keep elements where expr is true |

### Buffer Functions

| Function | Returns | Description |
|---|---|---|
| `line(".")` | `Number` | Current line number |
| `col(".")` | `Number` | Current column number |
| `getline(n)` | `String` | Content of line n |
| `bufname("%")` | `String` | Current buffer name |
| `bufnr("%")` | `Number` | Current buffer number |

### File Functions

| Function | Returns | Description |
|---|---|---|
| `expand("%")` | `String` | Current filename |
| `expand("%:p")` | `String` | Absolute path |
| `expand("%:t")` | `String` | Tail (filename only) |
| `expand("%:r")` | `String` | Root (no extension) |
| `expand("%:e")` | `String` | Extension |

### Time Functions

| Function | Returns | Description |
|---|---|---|
| `strftime(fmt)` | `String` | Format current time |
| `localtime()` | `Number` | Unix timestamp |

## Variables

Variables accessible in the expression language.

### Special Variables (normative)

| Variable | Type | Description |
|---|---|---|
| `v:count` | `Number` | Operator count (0 if none) |
| `v:count1` | `Number` | Operator count (1 if none) |
| `v:register` | `String` | Pending register name |
| `v:version` | `Number` | Editor version |
| `v:true` | `Boolean` | Boolean true |
| `v:false` | `Boolean` | Boolean false |
| `v:null` | `Null` | Null value |

### Environment Variables

Access via `$NAME` syntax. `$HOME` returns the home directory path.

### Options as Variables

Access options via `&option` syntax. `&tabstop` returns the tabstop value.

## Command Line Evaluation

`:echo expression` evaluates and prints an expression. `:let var = expression` assigns the result to a variable.

## Related

- Expression register: [/docs/spec/editing/registers/expression-register.md](/docs/spec/editing/registers/expression-register.md)
- Expression mappings: [/docs/spec/scripting/mappings/expr-mappings.md](/docs/spec/scripting/mappings/expr-mappings.md)
