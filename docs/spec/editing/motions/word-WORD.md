# Word vs WORD Distinction

Two word boundary definitions.

## Overview

Vim distinguishes between:
- **word**: Letters, digits, underscores
- **WORD**: Non-whitespace characters

## word (Lowercase)

### Definition

A sequence of:
- Letters (a-z, A-Z)
- Digits (0-9)
- Underscores (_)

### Motions

| Motion | Description |
|--------|-------------|
| `w` | Next word start |
| `e` | Current word end |
| `b` | Previous word start |
| `ge` | Previous word end |

### Text Objects

| Object | Description |
|--------|-------------|
| `iw` | Inner word |
| `aw` | A word (with space) |

### Example


Five words in this string.

## WORD (Uppercase)

### Definition

A sequence of non-whitespace characters.
Only spaces, tabs, newlines separate WORDs.

### Motions

| Motion | Description |
|--------|-------------|
| `W` | Next WORD start |
| `E` | Current WORD end |
| `B` | Previous WORD start |
| `gE` | Previous WORD end |

### Text Objects

| Object | Description |
|--------|-------------|
| `iW` | Inner WORD |
| `aW` | A WORD (with space) |

### Example


Two WORDs in this string.

## Comparison

### Same Text


**words**: `func`, `arg1`, `arg2` (3 words)
**WORDs**: `func(arg1,`, `arg2)` (2 WORDs)

### Path Example


**words**: home, user, config, app, settings, json
**WORDs**: Entire path is one WORD

## When to Use word

### Good For

- Editing individual identifiers
- Navigating code tokens
- Precise word-by-word editing

### Examples


## When to Use WORD

### Good For

- Navigating paths/URLs
- Editing entire arguments
- Moving through long identifiers

### Examples


## Practical Comparison

### Delete Example


### Movement Example


## Programming Context

### Variable Names


### Method Chains


### URLs


## Text Objects

### word Object


### WORD Object


## Configuration
