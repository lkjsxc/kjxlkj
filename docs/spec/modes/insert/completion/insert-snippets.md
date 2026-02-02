# Snippet Expansion

Template-based code insertion.

## Overview

Snippets expand short triggers
into larger code templates.

## Basic Usage

### Trigger


### Example


Cursor at first placeholder.

## Snippet Structure

### Basic Snippet


### Parts

- `prefix`: Trigger text
- `body`: Expanded content
- `$1, $2`: Tab stops
- `$0`: Final cursor

## Tab Stops

### Navigation


### Order

Visit in numerical order:
`$1` → `$2` → `$3` → `$0`

### Final Position

`$0` is final cursor position.

## Placeholders

### With Default


### Example


### Overwrite

Type to replace placeholder text.

## Choice Placeholders

### Syntax


### Behavior

Shows dropdown with choices.

### Example


## Variable Substitution

### Built-in Variables


### Example


## Mirror Placeholders

### Same Value


### Behavior

Both update simultaneously.

### Example


## Transform Placeholders

### Syntax


### Example


Typing "Hello" produces:

## Snippet File Format

### JSON


### TOML


## File Location
