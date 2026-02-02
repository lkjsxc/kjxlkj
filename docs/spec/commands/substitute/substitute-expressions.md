# Substitute Expressions

Dynamic replacements with \=.

## Overview

The `\=` replacement syntax
allows evaluating expressions
for dynamic replacements.

## Basic Syntax


The expression result becomes
the replacement text.

## Simple Expressions

### Literal Values


### Arithmetic


## Submatch Function

### Access Matched Text


### Example

Multiply captured number by 10.

## String Functions

### Transformation


### Reversal


## Date/Time

### Current Date


### Format Codes

| Code | Output          |
|------|-----------------|
| `%Y` | Year (4 digit)  |
| `%m` | Month (01-12)   |
| `%d` | Day (01-31)     |
| `%H` | Hour (00-23)    |
| `%M` | Minute (00-59)  |
| `%S` | Second (00-59)  |

## Line Information

### Line Number


### Line Content


## Counter Variable

### Sequential Numbers


Better approach:

### Padded Numbers


## Register Content

### Insert Register


## Conditional Logic

### Ternary


### Complex Condition


## List Operations

### Join


"a" → "a-a"

### Split and Process


## Math Functions

### Arithmetic


### Formatting

Convert cents to dollars.

## String Manipulation

### Repeat

"a" → "aaa"

### Substring

Truncate to 3 chars.

### Padding

