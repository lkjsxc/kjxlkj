# Tree-sitter Text Objects

Syntax-aware text object selection.

## Overview

Tree-sitter provides accurate AST-based
text objects that understand code structure.

## Benefits

### Over Regex

- Accurate syntax understanding
- No false positives in strings
- Handles complex nesting
- Language-aware boundaries

### Examples


## Configuration

### Enable Tree-sitter


### Object Mappings


## Standard Objects

### Function


### Class


### Parameter


### Comment


### Block


## Language Queries

### Query Files

Located in:

### Query Syntax


## Common Captures

### Statement


### Conditional


### Loop


### Call


## Custom Queries

### Add Query File

Create `queries/{lang}/textobjects.scm`:


### Inline Configuration


## Node Selection

### Current Node


### Parent Node


### Sibling Navigation


## Incremental Selection

### Configuration


### Usage

Press repeatedly to expand:
