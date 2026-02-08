# User Functions

Back: [docs/spec/scripting/README.md](docs/spec/scripting/README.md)

User-defined commands and expressions.

## Overview

Users can define custom commands that bundle multiple
ex commands into a single named command. Functions
provide computed values for expression contexts.

## User Commands

### Definition

`:command {name} {replacement}` defines a new ex
command. Command names MUST start with an uppercase
letter to distinguish from built-in commands.

### With Arguments

`:command -nargs=1 Greet echo "Hello " . <args>`

| Nargs | Meaning |
|-------|---------|
| 0 | No arguments (default) |
| 1 | Exactly one argument |
| * | Any number of arguments |
| ? | Zero or one argument |
| + | One or more arguments |

### Special Tokens

| Token | Replacement |
|-------|-------------|
| `<args>` | All arguments as string |
| `<q-args>` | All arguments, quoted |
| `<f-args>` | Arguments as function args |
| `<line1>` | First line of range |
| `<line2>` | Last line of range |
| `<count>` | Count value |
| `<bang>` | `!` if bang was used |
| `<reg>` | Register name if given |

### Range Support

`:command -range Cmd {replacement}` allows the
command to accept a range.

| Flag | Meaning |
|------|---------|
| `-range` | Allow range (default: current line) |
| `-range=%` | Default range is whole file |
| `-range={n}` | Default count is n |
| `-count={n}` | Accept a count (default n) |

### Completion

`:command -complete=file Cmd ...` specifies
completion type for arguments.

| Type | Completes |
|------|-----------|
| `file` | File paths |
| `dir` | Directories |
| `buffer` | Buffer names |
| `command` | Ex command names |
| `option` | Option names |
| `color` | Color scheme names |
| `help` | Help tags |
| `custom,{fn}` | Custom function |

### Listing

`:command` lists all user-defined commands.
`:command {name}` shows definition of specific command.

### Removal

`:delcommand {name}` removes a user command.
`:comclear` removes all user commands.

## Expression Evaluation

### Let Command

`:let {var} = {expr}` sets a variable to the
result of an expression.

### Variable Scopes

| Prefix | Scope |
|--------|-------|
| `g:` | Global |
| `b:` | Buffer-local |
| `w:` | Window-local |
| `l:` | Function-local |
| `v:` | Vim special variables |

### Expression Types

| Type | Example |
|------|---------|
| String | `"hello"` |
| Number | `42` |
| Float | `3.14` |
| List | `[1, 2, 3]` |
| Boolean | `v:true`, `v:false` |

### String Operations

| Operation | Syntax |
|-----------|--------|
| Concatenation | `"a" . "b"` |
| Length | `strlen("abc")` |
| Substring | `strpart("abc", 1, 2)` |
| Match | `"abc" =~ "b"` |

## Built-in Functions

### Common Functions

| Function | Description |
|----------|-------------|
| `expand('%')` | Current file path |
| `line('.')` | Current line number |
| `col('.')` | Current column number |
| `bufnr('%')` | Current buffer number |
| `winnr()` | Current window number |
| `mode()` | Current mode string |
| `getline(n)` | Get line n content |
| `setline(n, s)` | Set line n to string s |

## Conditional Execution

`:if {expr}` / `:elseif` / `:else` / `:endif`
provides conditional command execution.

## Related

- Mappings: [docs/spec/scripting/mappings/README.md](docs/spec/scripting/mappings/README.md)
- Execute command: [docs/spec/commands/execution/execute-command.md](docs/spec/commands/execution/execute-command.md)
- Expression mappings: [docs/spec/scripting/mappings/expr-mappings.md](docs/spec/scripting/mappings/expr-mappings.md)
