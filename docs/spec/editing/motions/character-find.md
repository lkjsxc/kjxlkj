# Character Find Motions

f, t, F, T character search.

## Overview

Find specific characters on
the current line only.

## Forward Find

### f (Find)


**Inclusive**: Cursor lands ON character.

### t (Till)


**Exclusive**: Cursor lands BEFORE character.

## Backward Find

### F (Find back)


**Inclusive**: Cursor lands ON character.

### T (Till back)


**Exclusive**: Cursor lands AFTER character.

## Examples

### Forward


### Backward


## With Operators

### Delete


### Yank


### Change


## Repeat Find

### Same Direction


### Opposite Direction


### Example


## Count

### Multiple Characters


## Visual Mode

### Select With f/t


### Extend Selection


## Line Scope

### Line Only

f/t/F/T only search current line.
Will not cross line boundaries.

### No Match

If character not found on line:
- Cursor stays in place
- Error message shown

## Special Characters

### Finding Punctuation


### Finding Spaces


## With Counts

### Practical Examples


## Common Patterns

### Delete to Character


### Change Inside

