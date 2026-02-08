# Jump Motions

Structural navigation commands.

## Overview

Jump to matching structures, sections, blocks,
and diagnostic/VCS locations. These motions add
entries to the jumplist.

## Percent Motion

### Match Pairs

`%` jumps to the matching bracket/brace/parenthesis.
Cursor must be on a bracket character, or `%` scans
forward to the first bracket on the current line.

### Supported Pairs

| Open | Close | Name |
|------|-------|------|
| `(` | `)` | Parentheses |
| `[` | `]` | Brackets |
| `{` | `}` | Braces |

### Nesting

`%` correctly handles nested pairs. It tracks depth
using a counter: increment on open, decrement on close.

### With Operators

`d%` deletes from cursor to matching bracket (inclusive).
`y%` yanks from cursor to matching bracket.
`v%` selects from cursor to matching bracket.

### matchpairs Option

The `matchpairs` option defines additional pairs.
Default: `(:),{:},[:]`. Add `<:>` for angle brackets:
`matchpairs = "(:),{:},[:]<:>"`.

## Section Motions

### Section Start

`[[` jumps backward to the previous `{` in column 1.
`]]` jumps forward to the next `{` in column 1.

### Section End

`[]` jumps backward to the previous `}` in column 1.
`][` jumps forward to the next `}` in column 1.

### Definition

A section boundary is a `{` or `}` that appears in
column 1 (the first character of the line). This
matches C-style function definitions.

### With Tree-sitter

When tree-sitter is available, section motions use
AST-based function/class boundaries instead of the
column-1 brace heuristic.

## Method Motions

### Jump to Method

`]m` jumps to start of next method/function.
`[m` jumps to start of previous method/function.
`]M` jumps to end of next method/function.
`[M` jumps to end of previous method/function.

### With Tree-sitter

Tree-sitter provides accurate method detection for
all supported languages. Without tree-sitter, these
fall back to section motions.

## Block Motions

### Unmatched Brackets

`[{` jumps to the previous unmatched `{`.
`]}` jumps to the next unmatched `}`.
`[(` jumps to the previous unmatched `(`.
`])` jumps to the next unmatched `)`.

### Use Case

Navigate to the enclosing scope from deep inside
nested blocks. Useful for finding the start of a
function or conditional block.

## Comment Motions

### Navigate Comments

`]//` or `]/` jumps to the start of the next comment.
`[//` or `[/` jumps to the start of the previous comment.

Tree-sitter provides accurate comment detection.

## Diff Motions

### Navigate Changes

`]c` jumps to the next diff/change hunk.
`[c` jumps to the previous diff/change hunk.

### In Diff Mode

In diff mode, these navigate between diff hunks.
In normal editing, they navigate between git-modified
regions (gutter signs).

## Error Motions

### Navigate Diagnostics

| Key | Action |
|-----|--------|
| `]d` | Next diagnostic (any severity) |
| `[d` | Previous diagnostic |
| `]e` | Next error |
| `[e` | Previous error |
| `]w` | Next warning |
| `[w` | Previous warning |

Diagnostics come from LSP, linters, or the compiler.

## Git Hunk Motions

### Navigate Git Changes

`]g` jumps to the next git change hunk.
`[g` jumps to the previous git change hunk.

These use the gutter diff signs to identify changed,
added, or deleted regions.

## Quickfix Motions

### Navigate Quickfix

`]q` jumps to the next quickfix entry.
`[q` jumps to the previous quickfix entry.
`]Q` jumps to the last quickfix entry.
`[Q` jumps to the first quickfix entry.

### Location List

`]l` jumps to the next location list entry.
`[l` jumps to the previous location list entry.

## Buffer Motions

### Navigate Buffers

`]b` jumps to the next buffer.
`[b` jumps to the previous buffer.
`]B` jumps to the last buffer.
`[B` jumps to the first buffer.

## Argument Motions

### Navigate Args

`]a` jumps to the next argument list file.
`[a` jumps to the previous argument list file.

## Tab Motions

### Navigate Tabs

`]t` or `gt` jumps to the next tab.
`[t` or `gT` jumps to the previous tab.
`{count}gt` jumps to tab number {count}.
