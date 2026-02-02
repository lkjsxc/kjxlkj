# Expression Mappings

Dynamic mappings using expressions.

## Overview

Expression mappings execute code and use
the result as the mapping's key sequence.

## Basic Syntax

### Expression Flag


### Inline Expression


## Visual Line Navigation

### Smart j/k


Move by display lines when no count given.

## Conditional Mappings

### Mode Check


### Buffer Check


## Available Functions

### State Functions

| Function | Returns |
|----------|---------|
| `pumvisible()` | Popup menu visible |
| `mode()` | Current mode |
| `visualmode()` | Last visual mode |
| `col(".")` | Current column |
| `line(".")` | Current line |

### Buffer Functions

| Function | Returns |
|----------|---------|
| `&filetype` | Buffer filetype |
| `&modified` | Buffer modified |
| `bufname()` | Buffer name |
| `winnr()` | Window number |

## Smart Tab

### Completion Aware


### Snippet Aware


## Smart Enter

### Completion Accept


### Auto-Close Aware


## Ternary Expressions

### Simple Ternary


### Nested Ternary


## Variables

### Access Variables


### Counts


## Register Expressions

### Dynamic Register


## Motion Expressions

### Conditional Motion


## Operator Expressions

### Smart Operator


## Insert Expressions

### Smart Backspace

