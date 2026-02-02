# Linewise vs Characterwise

Motion scope types.

## Overview

Motions operate on either:
- Characters (characterwise)
- Whole lines (linewise)

## Characterwise Motions

### Definition

Operates on individual characters.
Partial lines are affected.

### Examples

| Motion | Description |
|--------|-------------|
| `h`, `l` | Left/right character |
| `w`, `e`, `b` | Word motions |
| `f`, `t`, `F`, `T` | Character find |
| `0`, `^`, `$` | Line positions |
| `/`, `?` | Search |
| `%` | Match bracket |

### Behavior


Only the specific characters deleted.

## Linewise Motions

### Definition

Operates on entire lines.
Full lines are affected.

### Examples

| Motion | Description |
|--------|-------------|
| `j`, `k` | Up/down |
| `{`, `}` | Paragraph |
| `[[`, `]]` | Section |
| `'mark` | Go to mark (line) |
| `H`, `M`, `L` | Screen position |
| `gg`, `G` | File position |

### Behavior


## Forced Scope

### Force Characterwise


### Force Linewise


### Force Blockwise


## Practical Examples

### Characterwise Delete


### Linewise Delete


## Visual Mode Scope

### Characterwise Visual


### Linewise Visual


### Conversion


## Operator Behavior

### Characterwise Operator


### Linewise Operator


## Text Objects

### Usually Linewise

| Object | Scope |
|--------|-------|
| `ap`, `ip` | Paragraph (linewise) |
| `a}`, `i}` | Braces (can be linewise) |

### Usually Characterwise

| Object | Scope |
|--------|-------|
| `aw`, `iw` | Word |
| `a"`, `i"` | Quotes |
| `at`, `it` | Tag |

## Yank and Put

### Characterwise Yank


Put characterwise yanked text:
- `p` puts after cursor
- `P` puts before cursor

### Linewise Yank


Put linewise yanked text:
- `p` puts below current line
- `P` puts above current line

## Register Info

### Check Scope


Shows `l` for linewise, `c` for characterwise.

### Type Indicator


