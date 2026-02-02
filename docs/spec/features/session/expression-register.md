# Expression Register

The expression register `=` for evaluating expressions.

## Overview

The expression register allows inserting the result
of evaluated expressions into text.

## Basic Usage

### Insert Mode


Type expression, press Enter to insert result.

### Normal Mode


## Supported Expressions

### Arithmetic


### String Operations


### Built-in Variables

| Variable | Description |
|----------|-------------|
| `line` | Current line number |
| `col` | Current column |
| `filename` | Current file name |
| `filetype` | Current file type |

### Examples


## Date and Time


## Random Values


## String Functions

| Function | Description |
|----------|-------------|
| `upper(s)` | Uppercase |
| `lower(s)` | Lowercase |
| `trim(s)` | Remove whitespace |
| `len(s)` | String length |
| `repeat(s, n)` | Repeat string |

## Use Cases

### Insert Date Header


### Line Numbers


### Generate Sequences

In macro, use expression with counter.

### Quick Math

Insert calculation results inline.

## Comparison with Vim

| Feature | Vim | kjxlkj |
|---------|-----|--------|
| VimL expressions | ✓ | ✗ |
| Basic math | ✓ | ✓ |
| Strings | ✓ | ✓ |
| Built-in vars | ✓ | Subset |
| Functions | All | Limited |

## Limitations

- No arbitrary code execution
- Limited function set
- No external commands
- No variable assignment

## Tips

1. Use for quick calculations
2. Insert timestamps in notes
3. Generate repetitive text
4. Access file metadata
