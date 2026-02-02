# Substitute Command

Search and replace operations.

## Overview

The substitute command replaces text
matching patterns with replacements.

## Basic Syntax


## Simple Replace

### First on Line


### All on Line


### Entire File


## Common Flags

| Flag | Description |
|------|-------------|
| `g` | Global (all on line) |
| `c` | Confirm each |
| `i` | Case insensitive |
| `I` | Case sensitive |
| `n` | Count only |
| `e` | No error if no match |

## Confirmation

### Confirm Each


Responses:
- `y` - Replace
- `n` - Skip
- `a` - Replace all
- `q` - Quit
- `l` - Replace and quit

## Range Examples


## Special Characters

### In Pattern

| Char | Meaning |
|------|---------|
| `.` | Any character |
| `*` | Zero or more |
| `\+` | One or more |
| `\?` | Zero or one |
| `\s` | Whitespace |
| `\d` | Digit |
| `\w` | Word char |

### In Replacement

| Char | Meaning |
|------|---------|
| `&` | Entire match |
| `\0` | Entire match |
| `\1` | First group |
| `\r` | Newline |
| `\t` | Tab |
| `\\` | Literal backslash |

## Groups

### Capture Groups


Swaps: `foobar` → `barfoo`

### Non-Capturing


## Case Modification

### In Replacement

| Modifier | Effect |
|----------|--------|
| `\u` | Uppercase next |
| `\l` | Lowercase next |
| `\U` | Uppercase following |
| `\L` | Lowercase following |
| `\e` | End case change |

### Examples


## Expressions

### Expression Replacement


Increment numbers.

### Functions


Replace with length.

## Delimiters

### Alternative Delimiters


Useful when pattern contains `/`.

## Escaping

### Literal Characters


## Common Patterns

### Remove Trailing Whitespace


### Remove Leading Whitespace


### Remove Empty Lines


### Compress Whitespace


### Add Line Numbers


