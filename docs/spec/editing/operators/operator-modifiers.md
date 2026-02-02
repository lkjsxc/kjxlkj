# Operator Modifiers

Forcing motion types with operators.

## Overview

Override the default motion type
when applying operators.

## Motion Type Basics

### Three Types

1. **Characterwise**: Individual characters
2. **Linewise**: Complete lines
3. **Blockwise**: Rectangular block

### Default Behavior

Motions have inherent types:
- `w`, `e`: characterwise
- `j`, `k`: linewise
- `<C-v>`: blockwise (visual)

## Force Characterwise

### Modifier


### Usage


### Effect

Normally `dj` is linewise (two lines).
`dvj` deletes from cursor to same
column on next line (characterwise).

### Example


With cursor on 'w':
- `dj` → deletes both lines
- `dvj` → deletes "world\nfoo "

## Force Linewise

### Modifier


### Usage


### Effect

Normally `dw` is characterwise.
`dVw` deletes entire lines covered.

### Example


With cursor on 'h':
- `dw` → "world"
- `dVw` → (entire line deleted)

## Force Blockwise

### Modifier


### Usage


### Effect

Creates rectangular selection
for the delete operation.

### Example


With cursor on 'w':
- `dj` → deletes lines
- `d<C-v>j` → deletes column

## Practical Examples

### Characterwise Force


Use when motion is linewise
but you want characters.

### Linewise Force


Use when motion is characterwise
but you want full lines.

### Blockwise Force


`d<C-v>2j$` deletes to end
of each line (block).

## With Text Objects

### Force on Objects


Usually not needed with objects,
but available for special cases.

## Motion Type Table

| Motion | Default      | v       | V        | Ctrl-V   |
|--------|-------------|---------|----------|----------|
| w      | char        | char    | line     | block    |
| j      | line        | char    | line     | block    |
| $      | char        | char    | line     | block    |
| G      | line        | char    | line     | block    |
| /pat   | char        | char    | line     | block    |

## Yank with Modifiers

### Force Types


### Paste Behavior

Yanked type affects paste.
Block yank → block paste.

## Change with Modifiers

### Force Types


### Insert Behavior

After forced change:
