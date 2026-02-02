# Regular Expressions

Pattern matching syntax.

## Overview

Regular expressions match text patterns
for searching and substitution.

## Basic Patterns

### Literal Text


### Case Sensitivity


## Metacharacters

### Any Character


### Character Classes


### Predefined Classes

| Pattern | Matches |
|---------|---------|
| `\d` | Digit [0-9] |
| `\D` | Non-digit |
| `\w` | Word char [a-zA-Z0-9_] |
| `\W` | Non-word char |
| `\s` | Whitespace |
| `\S` | Non-whitespace |
| `\a` | Alpha [a-zA-Z] |
| `\l` | Lowercase |
| `\u` | Uppercase |

## Quantifiers

### Basic

| Pattern | Meaning |
|---------|---------|
| `*` | Zero or more |
| `\+` | One or more |
| `\?` | Zero or one |
| `\{n}` | Exactly n |
| `\{n,}` | n or more |
| `\{n,m}` | n to m |

### Non-Greedy

| Pattern | Meaning |
|---------|---------|
| `\{-}` | Zero or more (minimal) |
| `\{-n,m}` | n to m (minimal) |

## Anchors

### Position

| Pattern | Meaning |
|---------|---------|
| `^` | Start of line |
| `$` | End of line |
| `\<` | Start of word |
| `\>` | End of word |
| `\zs` | Set match start |
| `\ze` | Set match end |

### Examples


## Groups

### Capturing Groups


### Non-Capturing


### Alternation


## Lookahead/Lookbehind

### Positive Lookahead


### Negative Lookahead


### Positive Lookbehind


### Negative Lookbehind


## Special Sequences

### Visual Selection


### Column Position


### Line Position


## Escaping

### Literal Metacharacters


## Very Magic

### Simplified Syntax


### Comparison

| Normal | Very Magic |
|--------|------------|
| `\(` | `(` |
| `\)` | `)` |
| `\|` | `|` |
| `\+` | `+` |
| `\?` | `?` |

## No Magic

### Literal Mode
