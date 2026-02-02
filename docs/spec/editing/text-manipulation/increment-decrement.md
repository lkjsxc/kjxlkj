# Increment/Decrement

Number and value manipulation.

## Overview

Quickly increment or decrement numbers
and other values under the cursor.

## Basic Operations

### Increment

| Key | Action |
|-----|--------|
| `Ctrl-A` | Increment by 1 |
| `{n}Ctrl-A` | Increment by n |

### Decrement

| Key | Action |
|-----|--------|
| `Ctrl-X` | Decrement by 1 |
| `{n}Ctrl-X` | Decrement by n |

## Examples

### Simple Increment

Before: `10`
After `Ctrl-A`: `11`

### With Count

Before: `10`
After `5Ctrl-A`: `15`

### Decrement

Before: `10`
After `Ctrl-X`: `9`

## Number Formats

### Decimal


### Hexadecimal


### Octal


### Binary


## Configuration

### Number Formats


### Options

| Format | Numbers |
|--------|---------|
| `bin` | Binary (0b...) |
| `hex` | Hexadecimal (0x...) |
| `octal` | Octal (0...) |
| `alpha` | Alphabetic (a-z) |

## Negative Numbers

### Works with Negatives

Before: `-5`
After `Ctrl-A`: `-4`

### Through Zero

Before: `-1`
After `2Ctrl-A`: `1`

## Visual Mode

### Increment Selection

Select numbers, then `Ctrl-A`:

Before:

After:

### Sequential Increment

Select numbers, then `g Ctrl-A`:

Before:

After:

## Block Mode

### Column Increment

`Ctrl-V` to select column of numbers:


After `g Ctrl-A`:

## Alphabetic

### Enable Alpha


### Increment Letters

Before: `a`
After `Ctrl-A`: `b`

### Wraparound

Before: `z`
After `Ctrl-A`: `aa` (with setting)

## Dates

### Date Increment


Before: `2024-01-01`
After `Ctrl-A`: `2024-01-02`

### Date Formats


## Boolean Toggle

### Toggle True/False

