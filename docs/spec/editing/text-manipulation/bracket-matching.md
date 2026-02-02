# Bracket Matching

Navigate matching pairs.

## Overview

Jump between matching brackets,
parentheses, and other pairs.

## Percent Motion

### Basic Usage


### Supported Pairs

| Open | Close |
|------|-------|
| `(` | `)` |
| `[` | `]` |
| `{` | `}` |
| `<` | `>` |

### Behavior

From opening bracket: jump to closing.
From closing bracket: jump to opening.

## Example


On `{` after `main()`, `%` jumps to final `}`.

## Extended Matching

### Language Keywords


### Configuration


## Matchit Equivalent

### Enable Extended


### Language-Specific


## Navigation

### Forward to Match


### Operators with %


## Finding Unmatched

### Unmatched Brackets


## Bracket Text Objects

### Objects

| Object | Description |
|--------|-------------|
| `i(`, `ib` | Inside parentheses |
| `a(`, `ab` | Around parentheses |
| `i[` | Inside brackets |
| `a[` | Around brackets |
| `i{`, `iB` | Inside braces |
| `a{`, `aB` | Around braces |
| `i<` | Inside angle brackets |
| `a<` | Around angle brackets |

### Examples


## Highlighting

### Match Highlight


### Colors


## Multi-Cursor Matching

### Select All Matches


### Add Cursor at Match


## Tree-sitter Matching

### Semantic Matching


Uses syntax tree for accurate
bracket/block matching.

### Benefits

- Language-aware matching
- Handles strings/comments
- Nested structure awareness

## Jump List

### Back to Origin
