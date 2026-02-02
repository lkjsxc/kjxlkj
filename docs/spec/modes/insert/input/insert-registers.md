# Insert Register Access

Inserting register content in insert mode.

## Overview

Access registers to paste content
without leaving insert mode.

## Basic Command

### Syntax


### Behavior

Inserts register content at cursor.

## Common Registers

### Unnamed Register


### Yank Register


### Named Registers


### Clipboard


## Special Registers

### Current File


### Alternate File


### Last Command


### Last Search


### Last Insert


### Small Delete


## Expression Register

### Evaluate Expression


### Examples


## Insert Modes

### Standard Insert


### Literal Insert


No interpretation of special chars.

### No Indent


### Fix Indent


## Differences

### <C-r>{reg}

- Interprets mappings
- Triggers abbreviations
- Applies autoindent

### <C-r><C-r>{reg}

- No mapping interpretation
- No abbreviation trigger
- Applies autoindent

### <C-r><C-o>{reg}

- Literal insert
- No autoindent
- Preserves format

## Word Under Cursor

### In Command Line


### In Insert Mode

Use expression register:

## Register Content

### Check Content

From normal mode:

### Preview in Insert

Not built-in, but:

## Multi-line Content

### Linewise Register


### Blockwise Register

Inserts block at cursor column.
