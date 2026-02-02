# Argument Text Objects

Text objects for function arguments.

## Overview

Select individual arguments in
function calls, definitions, or
comma-separated lists.

## Inner Argument

### Command


### Behavior

Selects a single argument.
Excludes commas and spaces.

### Example


## Around Argument

### Command


### Behavior

Includes comma and space.
Smart about which side.

### Example


## Smart Separator Handling

### Middle Argument


### Last Argument


### First Argument


### Single Argument


## Detection Algorithm

### Finding Arguments

1. Find enclosing brackets
2. Split by top-level commas
3. Determine current argument
4. Handle nested structures

### Respects Nesting


## Complex Arguments

### Function Calls


### Objects/Arrays


### Closures


## Generic Arguments

### Type Parameters


### Template Arguments


## Array Elements

### List Items


### Works Similarly

Same comma-separation logic.

## Object Properties

### Key-Value


### Smart Handling

Treats `key: value` as unit.

## Multiline Arguments

### Formatted Calls


With cursor on arg2:
