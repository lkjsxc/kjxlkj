# Range Commands

Line range specifications.

## Overview

Ranges specify which lines a command
operates on.

## Basic Ranges

### Line Numbers


### Current Line


### Last Line


### All Lines


## Range Specifiers

### Absolute

| Specifier | Meaning |
|-----------|---------|
| `n` | Line number n |
| `.` | Current line |
| `$` | Last line |
| `%` | All lines (1,$) |

### Relative

| Specifier | Meaning |
|-----------|---------|
| `+n` | n lines down |
| `-n` | n lines up |
| `.+3` | Current + 3 |
| `.-2` | Current - 2 |

### Examples


## Marks

### Mark Range


### Visual Selection


Visual mode automatically sets `'<` and `'>`.

## Patterns

### Search Pattern


### Pattern Range


### With Offset


## Global Pattern

### Apply to Matches


### Apply to Non-Matches


## Combining Ranges

### With Offset


### Multiple Sections


## Common Commands with Ranges

### Substitute


### Copy


### Move


### Join


### Write


### Read


### Print


## Visual Ranges

### After Visual Selection

Exit visual mode, then:


### Within Visual Mode


## Line Offsets

### From Mark

