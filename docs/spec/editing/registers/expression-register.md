# Expression Register

The `=` register for evaluating expressions and inserting results.

## Basic Usage

### In Normal Mode


### In Insert Mode


### In Command Line


## Expression Syntax

### Arithmetic

| Expression | Result |
|------------|--------|
| `2+3` | 5 |
| `10-4` | 6 |
| `3*4` | 12 |
| `15/4` | 3 (integer) |
| `15.0/4` | 3.75 (float) |
| `17%5` | 2 (modulo) |
| `2**10` | 1024 (power) |

### String Operations

| Expression | Result |
|------------|--------|
| `"hello"` | hello |
| `"a" . "b"` | ab (concat) |
| `strlen("text")` | 4 |
| `toupper("hi")` | HI |
| `tolower("HI")` | hi |
| `substitute("foo", "o", "a", "g")` | faa |

### Register Access

| Expression | Result |
|------------|--------|
| `@a` | Contents of register a |
| `@"` | Unnamed register |
| `@0` | Yank register |
| `@/` | Search pattern |
| `@%` | Current filename |

## Common Functions

### String Functions


### Number Functions


### Date/Time


### Environment


## Conditional Expressions


## List Operations


## Practical Examples

### Insert Line Number


### Insert Date


### Calculate and Insert


### Transform Register Content


### Counter Pattern


## Expression History

Navigate previous expressions:

| Key | Action |
|-----|--------|
| `↑` | Previous expression |
| `↓` | Next expression |
| `<C-p>` | Previous |
| `<C-n>` | Next |

## Configuration


## Error Handling

Invalid expressions show error:


## API Reference

