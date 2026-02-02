# Forced Motion Types

Explicitly set motion behavior.

## Overview

Override how motions interact
with operators by forcing the
selection type.

## Characterwise Force

### Syntax


### Purpose

Make a linewise motion act
as characterwise.

### Example


Cursor on 'h':
- `d2j` → deletes 3 lines
- `dv2j` → deletes from 'h' to 'b'

## Linewise Force

### Syntax


### Purpose

Make a characterwise motion
act as linewise.

### Example


Cursor on 'w':
- `d$` → deletes "world more"
- `dV$` → deletes entire line

## Blockwise Force

### Syntax


### Purpose

Create rectangular selection
for any motion.

### Example


Cursor on 'c':
- `d2j` → deletes 3 lines
- `d<C-v>2j` → deletes column c-o

## Forcing Exclusive/Inclusive

### Exclusive Motions

End character not included:

### Inclusive Motions

End character included:

### Force Inclusive


Some interpretations vary.

## Applying to Operators

### Delete


### Yank


### Change


## Motion Categories

### Inherently Linewise

These motions are linewise:

### Inherently Characterwise

These motions are characterwise:

## Force Impact Table

| Motion | Type    | +v      | +V      | +Ctrl-V |
|--------|---------|---------|---------|---------|
| j      | line    | char    | line    | block   |
| w      | char    | char    | line    | block   |
| G      | line    | char    | line    | block   |
| $      | char    | char    | line    | block   |
| fe     | char    | char    | line    | block   |

## Exclusive vs Inclusive

### Default Behavior

- `w`: exclusive (stops before next word)
- `e`: inclusive (includes end char)
- `$`: inclusive (includes newline)

### Force Effects

Force modifier affects type,
not exclusivity.

## Practical Scenarios

### Copy Column


### Delete Non-Line Range


### Change Full Lines


## With Counts

