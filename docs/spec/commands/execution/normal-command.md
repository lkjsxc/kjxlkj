# Normal Command

Execute normal mode commands from Ex.

## Overview

The `:normal` command executes
normal mode commands on lines.

## Basic Syntax


## Simple Examples

### Single Command


### Multiple Commands


## With Range

### Multiple Lines


### Pattern Range


## Bang Modifier (!)

### Ignore Mappings


Without `!`, custom mappings apply.

### Recommended

Always use `!` in scripts
for predictable behavior.

## Special Keys

### Escape Sequence

Cannot type <Esc> directly.


### With Execute


## Common Uses

### Comment Lines


### Append to Lines


### Delete Characters


### Indent


## With Global

### Process Matching Lines


### Complex Actions

Yank function name.

## Macros

### Execute Macro


### Record + Apply

1. `qa` - Start recording
2. Edit one line
3. `q` - Stop recording
4. `:%normal! @a` - Apply to all

## Motion Commands

### Navigate and Act


### Search


## Insert Mode

### Enter and Exit


### Practical


## Visual Mode

### Select and Act


### Block Operations


## Operators

### Delete

