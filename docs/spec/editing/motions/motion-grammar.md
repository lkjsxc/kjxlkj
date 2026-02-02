# Motion Grammar

Understanding Vim motion structure.

## Overview

Motions follow a composable grammar:
`[count][operator][motion]`

## Basic Structure

### Components

| Component | Description | Example |
|-----------|-------------|---------|
| Count | Repetition | `3` |
| Operator | Action | `d`, `y`, `c` |
| Motion | Movement | `w`, `$`, `}` |

### Examples


## Operator-Motion Combination

### Pattern


### Common Operators

| Operator | Action |
|----------|--------|
| `d` | Delete |
| `y` | Yank (copy) |
| `c` | Change |
| `>` | Indent right |
| `<` | Indent left |
| `=` | Auto-indent |
| `gq` | Format |
| `g~` | Toggle case |
| `gu` | Lowercase |
| `gU` | Uppercase |

## Count Placement

### Count Before Operator


### Count Before Motion


### Both Counts


## Motion Types

### Character Motions

| Motion | Movement |
|--------|----------|
| `h` | Left |
| `l` | Right |
| `j` | Down |
| `k` | Up |

### Word Motions

| Motion | Movement |
|--------|----------|
| `w` | Word start |
| `e` | Word end |
| `b` | Word back |
| `W` | WORD start |
| `E` | WORD end |
| `B` | WORD back |

### Line Motions

| Motion | Movement |
|--------|----------|
| `0` | Line start |
| `^` | First non-blank |
| `$` | Line end |
| `g_` | Last non-blank |

### Sentence/Paragraph

| Motion | Movement |
|--------|----------|
| `(` | Sentence back |
| `)` | Sentence forward |
| `{` | Paragraph back |
| `}` | Paragraph forward |

## Text Objects

### Structure


### Inner vs Outer

| Prefix | Description |
|--------|-------------|
| `i` | Inner (inside) |
| `a` | A/around (including) |

### Objects

| Object | Description |
|--------|-------------|
| `w` | Word |
| `W` | WORD |
| `s` | Sentence |
| `p` | Paragraph |
| `[`, `]` | Brackets |
| `(`, `)` | Parentheses |
| `{`, `}` | Braces |
| `<`, `>` | Angle brackets |
| `"` | Double quotes |
| `'` | Single quotes |
| `` ` `` | Backticks |
| `t` | HTML/XML tag |

### Examples


## Doubled Operators

### Line Operation


## Motion Scope

### Characterwise

Operates on individual characters.
Most motions are characterwise.

### Linewise

Operates on entire lines.
`j`, `k`, `{`, `}` are linewise.

### Blockwise

Operates on rectangular block.
Visual block mode `<C-v>`.

## Exclusive vs Inclusive

### Exclusive

Doesn't include destination character.
`w`, `b`, `{`, `}` are exclusive.

### Inclusive

Includes destination character.
`e`, `f`, `t` are inclusive.

### Force Inclusive/Exclusive

