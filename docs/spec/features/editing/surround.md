# Surround Operations

kjxlkj supports vim-surround style operations.

## Overview

Easily add, change, or delete surrounding pairs.

## Adding Surroundings

Wrap text in delimiter pairs.

### With `ys` (you surround)


Examples:
- `ysiw"` - Surround word with quotes: `word` → `"word"`
- `ys$)` - Surround to end of line with parens
- `yss}` - Surround entire line with braces

### Visual Mode


Select text, press `S"` to add quotes.

## Deleting Surroundings


Examples:
- `ds"` - Delete surrounding quotes: `"word"` → `word`
- `ds)` - Delete surrounding parens
- `dst` - Delete surrounding HTML tag

## Changing Surroundings


Examples:
- `cs"'` - Change `"word"` to `'word'`
- `cs)'` - Change `(word)` to `{word}`
- `cst<div>` - Change tag to div

## Pair Aliases

| Alias | Open | Close |
|-------|------|-------|
| `b` or `)` | `(` | `)` |
| `B` or `}` | `{` | `}` |
| `r` or `]` | `[` | `]` |
| `a` or `>` | `<` | `>` |
| `t` | HTML tag | |
| `` ` `` | `` ` `` | `` ` `` |
| `"` | `"` | `"` |
| `'` | `'` | `'` |

## Spacing

Opening bracket adds space:
- `ysiw(` → `( word )`
- `ysiw)` → `(word)`

## HTML Tags

Surround operations with HTML/XML tag pairs.

### Surround with Tag

- `ysiw<em>` → `<em>word</em>`
- `ysit<span class="foo">` → Full tag with attributes

### Change Tag

- `cst<div>` - Change any tag to div

### Delete Tag

- `dst` - Delete surrounding tag pair

## Function Calls


- `ysiwfprint` → `print(word)`

## Custom Surrounds

Users can define custom surround pairs in configuration.

## Multiline

Surround preserves indentation:


## Integration

Surround works with text objects and the dot-repeat system.

### With Text Objects

Works with all text objects:
- `ysip"` - Surround inner paragraph
- `ysa"'` - Surround around quotes

### With Repeat

`.` repeats last surround operation.
