# Grouping and Backreferences

Capturing groups and referencing matched content in patterns.

## Basic Grouping

### Creating Groups


### Group Usage


## Non-Capturing Groups

Groups that don't capture:


### Use Cases


## Backreferences

Reference captured groups in pattern:

| Reference | Meaning |
|-----------|---------|
| `\1` | First captured group |
| `\2` | Second captured group |
| `\3` | Third captured group |
| `\n` | Nth captured group |

### Examples


## Backreferences in Substitution


This swaps two words: "hello world" → "world hello"

### Substitution References

| Reference | Meaning |
|-----------|---------|
| `\0` or `&` | Entire match |
| `\1` - `\9` | Captured groups |
| `\r` | Newline in replacement |
| `\t` | Tab in replacement |

## Very Magic Backreferences


## Nested Groups

Groups are numbered by opening parenthesis:



## Complex Examples

### Match Duplicate Words


Matches: "the the", "is is"

### Match Balanced Quotes


Matches: "hello", 'world'

### Match Repeated Pattern


Matches: "12-12-12"

### Match HTML Tag Pair


Matches: `<div>content</div>`

## Substitution Examples

### Swap Order


### Duplicate Content


### Wrap in Tags


### Case Conversion


## Group Modifiers

### Case Modifiers in Replacement

| Modifier | Effect |
|----------|--------|
| `\u` | Next char uppercase |
| `\U` | Following chars uppercase |
| `\l` | Next char lowercase |
| `\L` | Following chars lowercase |
| `\e` or `\E` | End case change |

### Examples


## Performance Notes

- Backreferences slower than non-capturing
- Use `\%(` when capture not needed
- Limit nesting depth for performance
- Avoid `\1` in character classes

## Common Patterns

| Task | Pattern |
|------|---------|
| Duplicate word | `\<\(\w\+\)\s\+\1\>` |
| Quoted string | `\(["']\).*\1` |
| Repeated char | `\(\w\)\1\+` |
| XML tag | `<\(\w\+\)>.*</\1>` |
| Variable assign | `\(\w\+\)\s*=\s*\1` |

## API Reference

