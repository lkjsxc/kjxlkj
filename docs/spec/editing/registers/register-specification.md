# Register Specification

Using registers with operators.

## Overview

Specify which register to use
when yanking, deleting, or pasting.

## Syntax

### Before Operator


### Examples


## Named Registers

### Lowercase (a-z)


Content replaces previous.

### Uppercase (A-Z)


Content appends to existing.

### Example


## Special Registers

### Unnamed Register


All yank/delete operations use this
unless specified otherwise.

### Clipboard Registers


### Example


## Read-Only Registers

### Current File


### Alternate File


### Last Command


### Last Insert


### Current Word


## Numbered Registers

### Yank History


### Behavior

- `"0`: Always last yank
- `"1-9`: Delete history stack

## Expression Register

### Evaluate


### Usage


### In Insert Mode


## Black Hole Register

### Discard


### Example


Use to avoid polluting registers.

## Small Delete Register

### Small Deletes


Deletes less than one line.

## Search Register

### Last Search


## With Operators

### Delete to Register


### Yank to Register

