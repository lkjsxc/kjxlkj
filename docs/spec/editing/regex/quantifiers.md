# Quantifiers

Pattern repetition modifiers for controlling match counts.

## Basic Quantifiers

| Quantifier | Meaning |
|------------|---------|
| `*` | Zero or more (greedy) |
| `\+` | One or more (greedy) |
| `\?` | Zero or one (greedy) |
| `\=` | Zero or one (synonym) |
| `\{n}` | Exactly n times |
| `\{n,}` | At least n times |
| `\{,m}` | At most m times |
| `\{n,m}` | Between n and m times |

## Greedy vs Non-Greedy

### Greedy (Default)

Match as much as possible:


### Non-Greedy

Match as little as possible:


| Greedy | Non-Greedy |
|--------|------------|
| `*` | `\{-}` |
| `\+` | `\{-1,}` |
| `\?` | `\{-0,1}` |
| `\{n,m}` | `\{-n,m}` |

## Very Magic Quantifiers

| Normal | Very Magic (\v) |
|--------|-----------------|
| `\+` | `+` |
| `\?` | `?` |
| `\{n,m}` | `{n,m}` |


## Count Examples

### Exact Count


### Minimum Count


### Maximum Count


### Range Count


## Non-Greedy Examples

### Basic Non-Greedy


### Non-Greedy with Count


## Possessive Quantifiers

Match without backtracking (fail fast):


## Combining Quantifiers

### With Character Classes


### With Groups


## Practical Examples

### Match IP Address


### Match Phone Number


### Match Email


### Match Quoted String


### Match HTML Tag


## Zero-Width Assertions

Quantifiers work with assertions:


## Common Patterns

| Task | Pattern |
|------|---------|
| Optional whitespace | `\s*` |
| Required whitespace | `\s\+` |
| Word characters | `\w\+` |
| Any content | `.*` |
| Minimal content | `.\{-}` |
| Fixed digits | `\d\{4}` |
| Variable length | `\w\{3,10}` |

## Performance Notes

- Non-greedy often slower
- Possessive faster but less flexible
- Specific counts faster than ranges
- Character classes faster than `.`

## Configuration

