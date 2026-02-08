# User-Defined Functions

Back: [/docs/spec/scripting/README.md](/docs/spec/scripting/README.md)

Users can define functions for use in expressions, mappings, and user commands.

## Definition (normative)

Functions are defined in command-file syntax:

| Syntax element | Description |
|---|---|
| `function! {Name}({args})` | Begin function definition (user functions MUST start with uppercase) |
| `return {expr}` | Return a value from the function |
| `endfunction` | End function definition |

## Parameters

| Feature | Syntax |
|---|---|
| Positional args | `function! MyFunc(a, b)` — accessed as `a:a`, `a:b` |
| Variadic | `function! MyFunc(...)` — accessed as `a:1`, `a:2`, etc. |
| Default values | Not supported natively; use `a:0` (arg count) to check |

## Scope

| Variable prefix | Scope |
|---|---|
| `l:` | Local to the function |
| `g:` | Global |
| `b:` | Buffer-local |
| `a:` | Function argument (read-only) |

Variables without a prefix default to local scope inside functions.

## Return values

Functions return a value via `return {expr}`. If no `return` is reached, the function returns 0.

## Calling functions

| Context | Syntax |
|---|---|
| From ex command | `:call MyFunc(args)` |
| In expression | `MyFunc(args)` (e.g., in `:echo`, `:let`, expression register) |
| From mapping | `:nnoremap <leader>x :call MyFunc()<CR>` |

## Error handling

If a function calls a command that fails, execution stops at that point unless the command was prefixed with `silent!`.

## Related

- User commands: [/docs/spec/scripting/user-commands.md](/docs/spec/scripting/user-commands.md)
- Expression register: [/docs/spec/editing/registers/expression-register.md](/docs/spec/editing/registers/expression-register.md)
- Event automation: [/docs/spec/scripting/event-automation.md](/docs/spec/scripting/event-automation.md)

