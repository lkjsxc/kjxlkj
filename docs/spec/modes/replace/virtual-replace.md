# Virtual Replace Mode

Tab-aware character replacement.

## Overview

Virtual replace mode considers
screen positions, not buffer bytes.

## Enter Virtual Replace

### Command


### Single Character


## Standard vs Virtual

### Standard Replace


### Virtual Replace


## Tab Handling

### Standard R

Replaces tab byte with one char.
Tab becomes single character.

### Virtual gR

Treats tab as multiple spaces.
Replacement preserves column width.

## Example

### Before


### Standard R + "ab"


### Virtual gR + "ab"


## Virtual Position

### Definition

Column position on screen,
not byte position in buffer.

### Impact

Multi-byte chars (tabs, wide Unicode)
counted by display width.

## Single Virtual Replace

### Command


### Behavior

Same as `gR`, but one character.

## Wide Characters

### Unicode

Wide characters (CJK, emoji)
occupy multiple columns.

### Virtual Replace


Respects visual width.

## Use Cases

### Tabular Data


Preserves column alignment.

### Fixed-Width

When visual alignment matters
more than byte positions.

### Source Code

When tabs represent indentation
and alignment matters.

## Backspace Behavior

### Restore Original


### Virtual Consideration

Restores correct number of
display columns.

## Exit Mode

### Commands


## Mode Indicator

### Display


### Cursor Shape


## Comparison Table

| Feature          | R      | gR            |
|-----------------|--------|---------------|
| Tab as          | 1 byte | display width |
| Wide char       | 1 byte | display width |
| Alignment       | broken | preserved     |
| Use case        | bytes  | visual        |

## Configuration

### Settings


### Tab Width

