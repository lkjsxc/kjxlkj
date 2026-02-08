# Tree-sitter Text Objects

Syntax-aware text object selection.

## Overview

Tree-sitter provides AST-based text objects that understand
code structure. Unlike regex-based text objects, these
never produce false positives inside strings or comments.

## Benefits

### Over Regex

- Accurate syntax understanding: parses actual grammar
- No false positives in strings, comments, or attributes
- Handles complex nesting (closures inside closures)
- Language-aware boundaries (e.g. Python indentation blocks)

### Examples

`daf` with tree-sitter correctly selects an entire Rust
closure including its capture list. Regex-based selection
would fail on nested braces.

## Configuration

### Enable Tree-sitter

Tree-sitter text objects are enabled by default when
a tree-sitter grammar is available for the current filetype.
Disable per-language in config TOML under `[languages.{lang}]`
with `treesitter_textobjects = false`.

### Object Mappings

| Key | Inner | Around | Object |
|-----|-------|--------|--------|
| `f` | `if` | `af` | Function / method |
| `c` | `ic` | `ac` | Class / struct / enum |
| `a` | `ia` | `aa` | Parameter / argument |
| `/` | `i/` | `a/` | Comment |
| `b` | `ib` | `ab` | Block (generic) |

These are configurable via TOML mappings.

## Standard Objects

### Function

`af` selects the entire function including signature, body,
and doc-comments/attributes. `if` selects only the body.
Works for `fn`, `def`, `function`, `func`, `method`, lambda.

### Class

`ac` selects the entire class/struct/enum including its body.
`ic` selects only the body. Supports Rust `struct`/`enum`/`impl`,
Python `class`, JS/TS `class`.

### Parameter

`aa` selects a function parameter including trailing comma
and whitespace. `ia` selects only the parameter text.

### Comment

`a/` selects a comment block (consecutive line comments or
a block comment). `i/` selects comment content without
delimiters (`//`, `/* */`, `#`).

### Block

`ab` selects a generic block (`{ }`, indentation block,
`do..end`). `ib` selects inner content.

## Language Queries

### Query Files

Tree-sitter text object queries are stored at:
`~/.config/kjxlkj/queries/{lang}/textobjects.scm`.
Built-in queries are bundled for common languages.

### Query Syntax

Queries use tree-sitter S-expression pattern language.
Capture names follow the convention `@{object}.inner`
and `@{object}.outer` (e.g. `@function.inner`).

## Common Captures

### Statement

`@statement.outer` captures a full statement including
semicolon or newline terminator.

### Conditional

`@conditional.outer` captures an if/else/match/switch block.
`@conditional.inner` captures the body.

### Loop

`@loop.outer` captures for/while/loop constructs.
`@loop.inner` captures the loop body.

### Call

`@call.outer` captures a function call expression including
arguments. `@call.inner` captures just the argument list.

## Custom Queries

### Add Query File

Create `~/.config/kjxlkj/queries/{lang}/textobjects.scm`
and define captures. The file extends (does not replace)
built-in queries. To override a built-in capture, use
`; inherits: false` at the top.

### Inline Configuration

Custom captures can also be defined in TOML config for
simple cases under `[languages.{lang}.textobjects]`.

## Node Selection

### Current Node

`<C-space>` (configurable) selects the AST node at
the cursor position as a visual selection.

### Parent Node

Repeating the select key expands to the parent node.
Useful for incremental selection widening.

### Sibling Navigation

`]a` / `[a` jump to the next/previous sibling node
at the same AST level. Works with parameter lists,
statement sequences, and array elements.

## Incremental Selection

### Configuration

Incremental selection keybindings are configured under
`[keys.normal]` in TOML. Default: `<C-space>` to start,
repeat to expand, `<BS>` to shrink.

### Usage

1. Press `<C-space>` to select identifier at cursor
2. Press again to expand to containing expression
3. Press again to expand to statement
4. Press again to expand to block
5. `<BS>` to shrink back one level

The selection stack is maintained as a `Vec<Range>`
that tracks each expansion level.
