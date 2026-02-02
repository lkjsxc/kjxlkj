# Pattern Anchors

Position-matching atoms for constraining where patterns match.

## Line Anchors

| Anchor | Meaning |
|--------|---------|
| `^` | Start of line |
| `$` | End of line |
| `\_^` | Start of line (multiline) |
| `\_$` | End of line (multiline) |

### Examples


## Word Boundaries

| Anchor | Meaning |
|--------|---------|
| `\<` | Start of word |
| `\>` | End of word |
| `\b` | Word boundary (either) |
| `\B` | Not a word boundary |

### Examples


## File Position Anchors

| Anchor | Meaning |
|--------|---------|
| `\%^` | Start of file |
| `\%$` | End of file |
| `\%#` | Cursor position |

### Examples


## Line Number Anchors

| Anchor | Meaning |
|--------|---------|
| `\%23l` | At line 23 |
| `\%>10l` | After line 10 |
| `\%<50l` | Before line 50 |

### Examples


## Column Anchors

| Anchor | Meaning |
|--------|---------|
| `\%5c` | At column 5 |
| `\%>10c` | After column 10 |
| `\%<80c` | Before column 80 |
| `\%5v` | At virtual column 5 |

### Examples


## Virtual Columns

Virtual columns account for tabs:


## Mark Positions

| Anchor | Meaning |
|--------|---------|
| `\%'a` | At mark 'a' |
| `\%>'m` | After mark 'm' |
| `\%<'z` | Before mark 'z' |

### Examples


## Visual Selection


## Match Boundaries

| Anchor | Meaning |
|--------|---------|
| `\zs` | Start of match |
| `\ze` | End of match |

### Examples


### Use Cases


## Combining Anchors

### Line Range


### Word at Position


## Very Magic Anchors

| Normal | Very Magic (\v) |
|--------|-----------------|
| `\<` | `<` |
| `\>` | `>` |
| `\%^` | `%^` |
| `\%$` | `%$` |
| `\zs` | `zs` (same) |
| `\ze` | `ze` (same) |


## Common Patterns

| Task | Pattern |
|------|---------|
| Start of line | `^` |
| End of line | `$` |
| Empty line | `^$` |
| Whole word | `\<word\>` |
| First line | `\%1l` |
| Last line | `\%$` |
| Column 80+ | `\%>80c` |
| Between marks | `\%'a.*\%'b` |

## API Reference

