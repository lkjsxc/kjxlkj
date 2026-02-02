# Exclusive vs Inclusive Motions

Motion boundary behavior.

## Overview

- **Inclusive**: Includes end character
- **Exclusive**: Excludes end character

## Inclusive Motions

### Definition

The destination character is included
in the operation.

### Examples

| Motion | Type | Description |
|--------|------|-------------|
| `e` | Inclusive | End of word |
| `ge` | Inclusive | End of previous word |
| `E` | Inclusive | End of WORD |
| `gE` | Inclusive | End of previous WORD |
| `$` | Inclusive | End of line |
| `g_` | Inclusive | Last non-blank |
| `f{c}` | Inclusive | Find character |
| `F{c}` | Inclusive | Find backward |
| `` ` `` | Inclusive | Go to mark |
| `%` | Inclusive | Match bracket |

### Behavior


## Exclusive Motions

### Definition

The destination character is NOT
included in the operation.

### Examples

| Motion | Type | Description |
|--------|------|-------------|
| `w` | Exclusive | Start of word |
| `W` | Exclusive | Start of WORD |
| `b` | Exclusive | Back to word start |
| `B` | Exclusive | Back to WORD start |
| `{` | Exclusive | Paragraph back |
| `}` | Exclusive | Paragraph forward |
| `(` | Exclusive | Sentence back |
| `)` | Exclusive | Sentence forward |
| `t{c}` | Exclusive | Till character |
| `T{c}` | Exclusive | Till backward |
| `/` | Exclusive | Search forward |
| `?` | Exclusive | Search backward |
| `n` | Exclusive | Next search |
| `N` | Exclusive | Previous search |

### Behavior


## Till vs Find

### Find (f) - Inclusive


### Till (t) - Exclusive


## Practical Examples

### Delete to Character


### Delete Word


## Force Inclusive

### Make Exclusive Inclusive


The `v` modifier forces characterwise
and makes exclusive motions inclusive.

### Example


## Special Cases

### Backward Motions

Backward motions work toward
line start, but rules still apply.


### Empty Result

Exclusive motion to current position
results in no operation.

## Line End Special

### $ Motion

`$` is inclusive but ignores
newline character for operations.


## Configuration

### Motion Definitions


## Visual Mode

### Selection Behavior

Inclusive motions extend selection
to include the destination.

Exclusive motions stop before
the destination character.

### Toggle Inclusive


## Operators Affected

### All Operators

- Delete (`d`)
- Yank (`y`)
- Change (`c`)
- Format (`gq`)
- Case (`gu`, `gU`, `g~`)
