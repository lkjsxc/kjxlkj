# Count with Operators

Using numeric counts with operators.

## Overview

Counts multiply the effect of
operators and motions.

## Basic Syntax

### Count Before Operator


### Count Before Motion


### Both


## Count Multiplication

### Example


### Rule

Counts multiply together.

## Count Positions

### Before Operator


### After Operator


### Equivalent


## With Line Operators

### Delete Lines


### Yank Lines


### Change Lines


## With Word Motions

### Word Operations


### WORD Operations


## With Line Motions

### Down


### Up


## Large Counts

### Big Numbers


### Practical Limits

- Stops at file/line end
- Undo as single operation

## Count with Text Objects

### Not Typical

Text objects don't usually take count.


### Use Repeat Instead


Better: use `.` to repeat

## Visual Mode Count

### After Selection


### In Visual


## Count Memory

### Repeat with Count


### Count Override

New count replaces remembered count.

## Zero Count

### Special Case


### Leading Zero


## Count and Registers

### Order

