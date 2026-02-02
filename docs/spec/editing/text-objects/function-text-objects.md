# Function Text Objects

Text objects for function definitions.

## Overview

Select function bodies, signatures,
or entire function definitions.

## Inner Function

### Command


### Behavior

Selects the body/implementation
without signature or delimiters.

### Example


`dif` removes:

## Around Function

### Command


### Behavior

Selects complete function:
signature + body + closing.

### Example


`daf` removes entire function.

## Language Support

### Rust


### Python


### JavaScript/TypeScript


### Go


## Methods

### Instance Methods


`af` on method selects entire method.

### Static Methods


## Closures/Lambdas

### Inner


`if` selects `x + 1`

### Around

`af` selects entire closure `|x| { x + 1 }`

### Python Lambda


`af` selects `lambda x: x + 1`

## Detection

### Tree-sitter Based


### Indent Based Fallback

For languages without tree-sitter:
- Find function start pattern
- Use indentation for body

## Nested Functions

### Outer/Inner


With cursor on `body`:
- `if` selects inner body
- `af` selects entire inner

### Navigate Up

Move cursor to outer function
to select outer function.

## Anonymous Functions

### JavaScript

