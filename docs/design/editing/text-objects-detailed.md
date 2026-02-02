# Text Objects

Operate on logical units of text.

## Overview

Text objects allow operations on semantic units
like words, sentences, paragraphs, and code blocks.

## Syntax


## Built-in Objects

### Word Objects

| Object | Description |
|--------|-------------|
| `w` | Word |
| `W` | WORD (space-separated) |

### Examples


### Block Objects

| Object | Description |
|--------|-------------|
| `(` `)` | Parentheses |
| `[` `]` | Brackets |
| `{` `}` | Braces |
| `<` `>` | Angle brackets |

### Examples


### Quote Objects

| Object | Description |
|--------|-------------|
| `"` | Double quotes |
| `'` | Single quotes |
| `` ` `` | Backticks |

### Examples


### Other Objects

| Object | Description |
|--------|-------------|
| `s` | Sentence |
| `p` | Paragraph |
| `t` | HTML/XML tag |

## Tree-sitter Objects

### Function


### Class


### Argument


### Comment


## Seeking

### Forward

Objects search forward if cursor not inside:


## Configuration


## Custom Objects

### Entire File


### Line


## Visual Mode

### Select Object


### Extend Selection


## Object Motion

### Around Objects


## Tips

1. Use `i` for content, `a` for delimiters
2. Tree-sitter objects are language-aware
3. Objects work with all operators
4. Visual mode to preview selection
