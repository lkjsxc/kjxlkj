# Multi-line Patterns

Searching and matching across line boundaries.

## Newline Matching

### Basic Newline

| Pattern | Meaning |
|---------|---------|
| `\n` | Match newline character |
| `\_s` | Whitespace including newline |
| `\_.` | Any character including newline |
| `\_^` | Start of line (multiline) |
| `\_$` | End of line (multiline) |

## Character Class Extensions

Add `\_` prefix for newline inclusion:

| Normal | With Newline |
|--------|--------------|
| `.` | `\_.` |
| `\s` | `\_s` |
| `\w` | `\_w` |
| `\a` | `\_a` |
| `[abc]` | `\_[abc]` |

## Multi-line Search Examples

### Match Across Lines


Matches "start" through "end" across any number of lines.

### Match Function Block


Matches function with body spanning multiple lines.

### Match Multi-line String


Matches Python triple-quoted string.

## Line-Oriented Patterns

### Consecutive Lines


Matches "line1" immediately followed by newline and "line2".

### Empty Lines


### Lines Starting With


Matches if/then on consecutive lines.

## Very Magic Multi-line


## Practical Examples

### Match Paragraph


Non-empty lines until blank line.

### Match Code Block


Markdown code block.

### Match Comment Block


### Match Function Definition


## Substitution Across Lines

### Join Lines


### Add After Pattern


### Multi-line Replace


## Using \_.* Carefully

### Greedy Behavior


### Performance

Multi-line patterns can be slow on large files:


## Visual Mode Multi-line


## Range with Newlines

### Match N Lines


### Fixed Line Range


## Joining Pattern

Match and join:


## Common Multi-line Patterns

| Task | Pattern |
|------|---------|
| C comment | `/\/\*\_.\{-}\*\/` |
| HTML tag | `/<\w\+\_.\{-}<\/\w\+>` |
| Triple string | `/"""\_.\{-}"""` |
| Empty lines | `/^\s*\n\s*$` |
