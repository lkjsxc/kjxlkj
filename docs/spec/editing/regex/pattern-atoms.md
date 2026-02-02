# Pattern Atoms

Regular expression atoms reference.

## Overview

Atoms are the basic building blocks
of regular expression patterns.

## Ordinary Atoms

### Single Character


### Any Character


## Character Classes

### Bracket Expression


### Predefined Classes

| Atom | Matches | Equivalent |
|------|---------|------------|
| `\d` | Digit | [0-9] |
| `\D` | Non-digit | [^0-9] |
| `\w` | Word char | [a-zA-Z0-9_] |
| `\W` | Non-word | [^a-zA-Z0-9_] |
| `\s` | Whitespace | [ \t\n\r] |
| `\S` | Non-space | [^ \t\n\r] |
| `\a` | Alpha | [a-zA-Z] |
| `\A` | Non-alpha | [^a-zA-Z] |
| `\l` | Lowercase | [a-z] |
| `\L` | Non-lower | [^a-z] |
| `\u` | Uppercase | [A-Z] |
| `\U` | Non-upper | [^A-Z] |
| `\x` | Hex digit | [0-9a-fA-F] |
| `\X` | Non-hex | [^0-9a-fA-F] |
| `\o` | Octal digit | [0-7] |
| `\O` | Non-octal | [^0-7] |

## Special Characters

### Escapes

| Atom | Character |
|------|-----------|
| `\t` | Tab |
| `\r` | Carriage return |
| `\n` | Newline |
| `\e` | Escape |
| `\b` | Backspace |

### Literal

| Atom | Character |
|------|-----------|
| `\\` | Backslash |
| `\.` | Period |
| `\*` | Asterisk |
| `\[` | Left bracket |
| `\]` | Right bracket |
| `\^` | Caret |
| `\$` | Dollar |

## Unicode

### Unicode Character


### Unicode Properties


## Position Atoms

### Line Position


### Word Position


### Column Position


### Line Number


### Virtual Column


## Match Position

### Zero-Width


### Example


## Grouping

### Capturing


### Non-Capturing


## Alternation

### Or


### In Very Magic


## Quantifiers

### Greedy

| Atom | Meaning |
|------|---------|
| `*` | 0 or more |
| `\+` | 1 or more |
| `\?` | 0 or 1 |
| `\{n}` | Exactly n |
| `\{n,}` | n or more |
| `\{,m}` | 0 to m |
| `\{n,m}` | n to m |

### Non-Greedy

| Atom | Meaning |
|------|---------|
