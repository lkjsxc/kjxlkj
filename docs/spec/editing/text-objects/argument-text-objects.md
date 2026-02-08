# Argument Text Objects

Select function arguments and parameters.

## Overview

Argument text objects select individual arguments in
function calls, definitions, and similar comma-separated
lists. They require tree-sitter for accurate detection.

## Commands

### Inner Argument

`ia` selects the argument content without surrounding
commas or whitespace.

### Around Argument

`aa` selects the argument including the trailing comma
and whitespace, or the leading comma if last argument.

## Examples

### Function Call

On `fn(alpha, beta, gamma)`, cursor on `beta`:
- `dia` deletes `beta`, result: `fn(alpha, , gamma)`
- `daa` deletes `beta, `, result: `fn(alpha, gamma)`

### First Argument

On `fn(alpha, beta)`, cursor on `alpha`:
- `daa` deletes `alpha, `, result: `fn(beta)`

### Last Argument

On `fn(alpha, beta)`, cursor on `beta`:
- `daa` deletes `, beta`, result: `fn(alpha)`

### Single Argument

On `fn(alpha)`, cursor on `alpha`:
- `daa` deletes `alpha`, result: `fn()`

## Supported Contexts

### Languages

Works in any language with tree-sitter support for
function calls and definitions.

### List Types

| Context | Example |
|---------|---------|
| Function call | `f(a, b, c)` |
| Function def | `fn f(x: i32, y: i32)` |
| Array literal | `[1, 2, 3]` |
| Tuple | `(a, b, c)` |
| Generic args | `HashMap<K, V>` |
| Template args | `std::vector<int, alloc>` |

## With Operators

### Common Operations

| Sequence | Effect |
|----------|--------|
| `dia` | Delete argument content |
| `daa` | Delete argument with comma |
| `cia` | Change argument content |
| `caa` | Change argument with comma |
| `yia` | Yank argument content |
| `via` | Select argument visually |

## Count

### Multiple Arguments

`d2aa` deletes 2 arguments including commas.
`v3ia` selects 3 argument contents.

## Fallback

### Without Tree-sitter

Without tree-sitter, argument detection falls back to
simple comma-based splitting within the nearest
parentheses. This is less accurate for nested calls.
