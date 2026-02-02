# Range Specifications

Defining line ranges for Ex commands.

## Overview

Ranges specify which lines
a command operates on.

## Basic Syntax


## Line Numbers

### Absolute


### Special Lines

| Symbol | Meaning         |
|--------|-----------------|
| `.`    | Current line    |
| `$`    | Last line       |
| `0`    | Before line 1   |
| `1`    | First line      |

### Examples


## Range Operators

### Comma (Inclusive)


### Semicolon (Set Position)


Difference from comma:

## Relative Addresses

### Plus/Minus


### Combined


## Percent Sign

### Entire File


## Visual Range

### Automatic

Entering `:` from visual mode:

### Marks

| Mark   | Meaning              |
|--------|---------------------|
| `'<`   | Start of visual     |
| `'>`   | End of visual       |

## Mark Ranges

### Single Marks


### Built-in Marks


## Pattern Ranges

### Forward Search


### Backward Search


### Pattern Offset


## Combined Patterns

### Examples


## Global Patterns

### With Global


### Range + Global


## Special Ranges

### None (Current)


### Count After


## Range Examples

### Common Tasks

