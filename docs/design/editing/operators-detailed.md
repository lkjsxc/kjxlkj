# Operators

Commands that operate on text.

## Overview

Operators work with motions and text objects:


## Core Operators

| Operator | Description |
|----------|-------------|
| `d` | Delete |
| `c` | Change |
| `y` | Yank (copy) |
| `>` | Indent right |
| `<` | Indent left |
| `=` | Auto-indent |
| `gq` | Format |
| `gu` | Lowercase |
| `gU` | Uppercase |
| `g~` | Toggle case |

## Delete

### Examples


### Registers

Deleted text goes to default register.

## Change

### Examples


### Enters Insert Mode

After change, you're in insert mode.

## Yank

### Examples


### Registers


## Indent

### Examples


### Configuration


## Format

### Examples


## Case Operators

### Examples


## Doubled Operators

Line-wise operation:

| Operator | Line Version |
|----------|--------------|
| `d` | `dd` |
| `c` | `cc` |
| `y` | `yy` |
| `>` | `>>` |
| `<` | `<<` |
| `=` | `==` |

## With Counts


## Visual Mode

### Apply to Selection


## Operator-Pending Mode

After pressing operator:
- Cursor blinks differently
- Waiting for motion/object
- `<Esc>` cancels

## Custom Operators

### Configuration


## Repeat

### Dot Operator

`.` repeats last operator + motion/object.

### Examples

