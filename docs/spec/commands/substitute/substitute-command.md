# Substitute Command

Find and replace text with :s.

## Overview

The substitute command replaces
text matching a pattern.

## Basic Syntax


## Simple Examples

### Basic Replace


### Current Line


## Range

### Common Ranges


### Patterns


## Delimiter

### Default Slash


### Alternatives

For paths with slashes:

## Flags

### Common Flags

| Flag | Meaning              |
|------|---------------------|
| `g`  | Global (all on line)|
| `c`  | Confirm each        |
| `i`  | Case insensitive    |
| `I`  | Case sensitive      |
| `n`  | Count only          |
| `e`  | No error if none    |
| `&`  | Reuse previous flags|

### Combinations


## Confirm Mode

### Interactive


Prompts:

### Confirm Options

| Key  | Action              |
|------|---------------------|
| `y`  | Replace this        |
| `n`  | Skip this           |
| `a`  | Replace all rest    |
| `q`  | Quit substituting   |
| `l`  | Replace and quit    |
| `^E` | Scroll up           |
| `^Y` | Scroll down         |

## Special Replacement

### Entire Match


### Captured Groups


### Numbered Groups


## Case Conversion

### Flags in Replacement

| Sequence | Effect              |
|----------|---------------------|
| `\u`     | Next char uppercase |
| `\U`     | Rest uppercase      |
| `\l`     | Next char lowercase |
| `\L`     | Rest lowercase      |
| `\e`     | End case change     |
| `\E`     | End case change     |

### Examples


## Empty Pattern

### Reuse Last Search


### After Search


## Empty Replacement

### Delete Matches


## Repeat Substitute

### Repeat Last


### Different Range


## Line Addressing

