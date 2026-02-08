# Recursive Macros

Back: [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)

Macros that call themselves for iterative processing.

## Overview

A macro can invoke itself to repeat until an error occurs (end of file, search fail, etc.). This is the primary mechanism for applying macros across an unknown number of targets.

## Pattern

1. Clear the register: `qaq` (record empty macro into `a`).
2. Record the macro with a self-call at the end: `qa{commands}@aq`.
3. Execute: `@a`.

## How It Terminates

The macro stops when any command in the sequence fails:

| Error | Example |
|---|---|
| End of file | `j` at last line |
| No search match | `/pattern` with no more matches |
| No more characters | `f{x}` with no `{x}` on line |

## Example

Delete all lines containing "TODO":

`qaq` `qa/TODO<CR>dd@aq` `@a`

This recursively searches for "TODO", deletes the line, and repeats until no more matches.

## Safety

Recursive macros have no built-in depth limit. They rely on command failure, so ensure at least one command will eventually fail.

## Related

- Macros: [/docs/spec/editing/macros/README.md](/docs/spec/editing/macros/README.md)
- Advanced macros: [/docs/spec/editing/macros/macros-advanced.md](/docs/spec/editing/macros/macros-advanced.md)
