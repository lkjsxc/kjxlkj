# Unicode Input

Entering Unicode characters in insert mode.

## Overview

Multiple methods for inserting
any Unicode character.

## Methods Summary

| Method          | Syntax              | Example      |
|-----------------|---------------------|--------------|
| Digraph         | `<C-k>{ab}`        | `<C-k>a:`→ä  |
| Hex 2-digit     | `<C-v>x{hh}`       | `x41`→A      |
| Unicode BMP     | `<C-v>u{hhhh}`     | `u03b1`→α    |
| Unicode Full    | `<C-v>U{hhhhhhhh}` | `U0001F600`→😀|
| Decimal         | `<C-v>{nnn}`       | `65`→A       |

## Unicode Code Points

### Basic Multilingual Plane


Range: U+0000 to U+FFFF

### Supplementary Planes


Range: U+10000 to U+10FFFF

## Common Unicode Ranges

### Latin Extended


Examples:

### Greek


Examples:

### Cyrillic


Examples:

### Mathematical


Examples:

### Arrows


Examples:

### Box Drawing


Examples:

### Currency


Examples:

### Emoji


Examples:

## Input Methods

### Quick Reference

Find code point:

### Unicode Name Search


### Emoji Picker


## Composed Characters

### Combining Diacritics


Example (a + combining acute):

### Precomposed


Prefer precomposed when available.

## Normalization

### Forms

- NFC: Composed (canonical)
- NFD: Decomposed

