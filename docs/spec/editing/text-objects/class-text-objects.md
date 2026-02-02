# Class Text Objects

Text objects for class definitions.

## Overview

Select class bodies, definitions,
or entire class structures.

## Inner Class

### Command


### Behavior

Selects class body/members
without class declaration.

### Example


`dic` removes methods and attrs,
leaving:

## Around Class

### Command


### Behavior

Selects complete class:
declaration + body + closing.

### Example


`dac` removes entire struct.

## Language Support

### Python


### Rust


### JavaScript/TypeScript


### Go


### Java/C#


## Structs and Types

### Rust Struct


Treated as class object.

### TypeScript Interface


Also matched by class object.

### Go Struct


## Enum Types

### Rust Enum


`ac` selects entire enum.
`ic` selects variants only.

### TypeScript Enum


## Nested Classes

### Inner Class


With cursor in Inner:
- `ic` selects Inner body
- `ac` selects entire Inner

### Navigate to Outer

Move cursor outside Inner
to operate on Outer.

## Impl Blocks

### Rust

