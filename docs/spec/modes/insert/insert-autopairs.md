# Auto-Pairs in Insert

Automatic bracket and quote pairing.

## Overview

Auto-pairs insert matching closing
characters when opening typed.

## Paired Characters

### Brackets


### Quotes


## Basic Behavior

### On Open

Typing `(` inserts `()` with
cursor between them.

### On Close

Typing `)` with cursor before `)`
skips over it (no duplicate).

### Example


## Skip Over

### When to Skip

If next char matches closing:

### Skip Works For

All configured pairs.

## Delete Pair

### Backspace


### Configuration


## Smart Quotes

### Context Awareness

Don't pair if:
- In string already
- In comment
- After word character

### Example


## Wrap Selection

### Visual Mode

Select text, type quote:

### Configuration


## Multi-character Pairs

### Examples


### Configuration


## Language-Specific

### Per Filetype


### Disabled Pairs


## Fast Wrap

### Jump Out


Or:

### Configuration


## Newline Between

### Smart Newline


### Configuration


## Space Between

### Smart Space


### Configuration

