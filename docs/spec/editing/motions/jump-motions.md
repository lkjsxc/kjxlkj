# Jump Motions

Structural navigation commands.

## Overview

Jump to matching structures,
sections, and blocks.

## Percent Motion

### Match Pairs


### Supported Pairs

- `()` Parentheses
- `[]` Brackets
- `{}` Braces
- `<>` Angle brackets

### Example


On `{` after `main()`, `%` jumps to matching `}`.

## Section Motions

### Section Start


### Section End


### Definition

Section starts at `{` in column 1.

### Code Example


## Method Motions

### Jump to Method


### With Tree-sitter


More accurate method detection.

## Block Motions

### Unmatched Brackets


### Use Case

Find enclosing scope from inside.


## Comment Motions

### Navigate Comments


## Diff Motions

### Navigate Changes


### In Diff Mode

Jump between diff hunks.

## Error Motions

### Navigate Diagnostics


## Git Hunk Motions

### Navigate Git Changes


## Quickfix Motions

### Navigate Quickfix


### Location List


## Buffer Motions

### Navigate Buffers


## Argument Motions

### Navigate Args


## Tab Motions

### Navigate Tabs

