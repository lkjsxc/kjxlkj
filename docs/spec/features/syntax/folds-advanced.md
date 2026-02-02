# Advanced Folds

Nested folds and custom folding.

## Overview

Advanced folding techniques for
complex code navigation.

## Nested Folds

### Fold Levels


### Navigate Levels


### Set Level


## Fold Methods

### Available Methods

| Method | Source |
|--------|--------|
| `manual` | User-defined folds |
| `indent` | Indentation levels |
| `expr` | Custom expression |
| `syntax` | Syntax regions |
| `marker` | Fold markers |
| `diff` | Unchanged regions |

### Configuration


## Expression Folds

### Basic Expression


### Example Function


### Return Values

| Value | Meaning |
|-------|---------|
| `0` | Not folded |
| `1`, `2`, ... | Fold level |
| `>1` | Start fold at level 1 |
| `<1` | End fold at level 1 |
| `=` | Same as previous line |
| `-1` | Undefined |

## Tree-sitter Folds

### Enable


### Fold Nodes


### Language Specific


## Marker Folds

### Syntax


### Custom Markers


## Fold Text

### Custom Display


### Format Tokens

| Token | Value |
|-------|-------|
| `{lines}` | Number of lines |
| `{level}` | Fold level |
| `{first_line}` | First line content |
| `{last_line}` | Last line content |

### Example Output


## Fold Options

### Configuration


### Per-Filetype


## Fold Commands

### Basic Commands

| Command | Action |
|---------|--------|
| `zo` | Open fold |
| `zc` | Close fold |
| `za` | Toggle fold |
| `zO` | Open recursively |
| `zC` | Close recursively |
| `zA` | Toggle recursively |
