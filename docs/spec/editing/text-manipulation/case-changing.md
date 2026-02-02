# Case Changing

Text case transformation.

## Overview

Change text case with operators and
commands for various case styles.

## Case Operators

### Toggle Case

| Key | Action |
|-----|--------|
| `~` | Toggle char under cursor |
| `g~{motion}` | Toggle motion |
| `g~~` | Toggle line |

### Uppercase

| Key | Action |
|-----|--------|
| `gU{motion}` | Uppercase motion |
| `gUU` | Uppercase line |

### Lowercase

| Key | Action |
|-----|--------|
| `gu{motion}` | Lowercase motion |
| `guu` | Lowercase line |

## Examples

### Toggle Word


Before: `Hello` → After: `hELLO`

### Uppercase Word


Before: `hello` → After: `HELLO`

### Lowercase Line


## Visual Mode

### Selection Case

Select text, then:

| Key | Action |
|-----|--------|
| `~` | Toggle case |
| `U` | Uppercase |
| `u` | Lowercase |

## Range Commands

### Uppercase Range


### Lowercase Range


### In Pattern


## Case Modifiers

### In Substitution

| Modifier | Effect |
|----------|--------|
| `\u` | Uppercase next char |
| `\l` | Lowercase next char |
| `\U` | Uppercase following |
| `\L` | Lowercase following |
| `\e` | End case modification |
| `\E` | End case modification |

### Examples


## Case Styles

### camelCase


Before: `hello_world` → After: `helloWorld`

### PascalCase


Before: `hello_world` → After: `HelloWorld`

### snake_case


Before: `helloWorld` → After: `hello_world`

### SCREAMING_SNAKE


Before: `helloWorld` → After: `HELLO_WORLD`

### kebab-case


Before: `helloWorld` → After: `hello-world`

### Title Case


Before: `hello world` → After: `Hello World`

### Sentence case


Before: `hello world` → After: `Hello world`

## Configuration

### Custom Commands


### Default Case


## Keybindings


## With Text Objects

### Uppercase Word

