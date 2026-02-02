# Magic Modes

Pattern interpretation modes.

## Overview

Magic modes control how special characters
are interpreted in patterns.

## Magic Levels

### Four Levels

| Mode | Flag | Description |
|------|------|-------------|
| Very Magic | `\v` | Most special chars active |
| Magic | `\m` | Default vim-like |
| No Magic | `\M` | Most literal |
| Very No Magic | `\V` | All literal |

## Magic (Default)

### Active Special Characters

| Char | Meaning |
|------|---------|
| `.` | Any character |
| `*` | Zero or more |
| `^` | Start of line |
| `$` | End of line |
| `[` | Character class |

### Escaped to Activate

| Pattern | Meaning |
|---------|---------|
| `\+` | One or more |
| `\?` | Zero or one |
| `\|` | Alternation |
| `\(` | Group start |
| `\)` | Group end |

### Example


## Very Magic (\v)

### Most Characters Special

After `\v`, these are special:
- `()` - Groups
- `|` - Alternation
- `+` - One or more
- `?` - Zero or one
- `{}` - Quantifiers

### Only Literal

- `_` - Underscore
- Letters and numbers

### Example


Same as `/\(foo\|bar\)\+` in magic mode.

### Comparison

| Magic | Very Magic |
|-------|------------|
| `\(foo\|bar\)` | `(foo\|bar)` |
| `\d\+` | `\d+` |
| `\w\{3,5\}` | `\w{3,5}` |
| `\<word\>` | `<word>` |

## No Magic (\M)

### Mostly Literal

Only `^` and `$` are special.

### Example


Matches literal `foo.bar`.

### Escape to Activate


`.` and `+` become special.

## Very No Magic (\V)

### All Literal

Everything is literal except `\`.

### Example


Matches literal `foo.bar` including the `.`.

### Escape to Activate


## Switching Modes

### Mid-Pattern


Switch to very magic mid-pattern.

### Multiple Switches


## Configuration

### Default Mode


### Per-Search

Override in search:


## Practical Use

### When to Use Very Magic

- Complex patterns with groups
- Multiple alternations
- Cleaner syntax for regexes

### When to Use No Magic

- Searching for literal text
- Text with many special chars
- File paths, URLs

## Examples

### Very Magic Email


### No Magic File Path


### Very No Magic URL


## Escape Reference

### In Magic Mode

| To Match | Pattern |
|----------|---------|
| `.` | `\.` |
| `*` | `\*` |
| `[` | `\[` |
| `^` | `\^` |
