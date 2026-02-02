# Code Folding

kjxlkj supports tree-sitter based code folding.

## Overview

Collapse code sections to improve navigation:


## Keybindings

### Basic Folding


### Global Folding


## Configuration


## Fold Methods

### Tree-sitter (Recommended)


Folds based on syntax:
- Functions
- Classes/structs
- Blocks
- Comments

### Indent-based


Folds based on indentation level.

### Manual


Create folds with `zf` + motion.

## Fold Display

### Fold Text


Customizable fold summary.

### Fill Character


## Language-Specific


## Navigation


## Fold Column

Visual indicator in gutter:



## Persistence

Save fold state:


## Commands

| Command | Description |
|---------|-------------|
| `:fold` | Create fold (visual mode) |
| `:foldopen` | Open all at cursor |
| `:foldclose` | Close all at cursor |

## Tree-sitter Queries

Custom fold queries per language:

