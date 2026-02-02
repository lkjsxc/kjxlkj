# File Type Detection

Automatic detection and configuration by file type.

## Overview

kjxlkj automatically detects file types and applies
appropriate settings and syntax highlighting.

## Detection Methods

### By Extension


### By Filename


### By Shebang


### By Content

First line patterns for detection.

## Configuration

### Extension Mapping


### Filename Mapping


### Pattern Mapping


## Shebang Detection


## Per-Filetype Settings

### Configuration


### Applied Settings

- Tab width
- Use spaces
- Formatters
- LSP servers
- Syntax highlighting

## Commands


## Override

### Per-Buffer


### By Modeline


## Detection Order

1. Modeline
2. Shebang
3. Filename
4. Extension
5. Content analysis

## Custom Filetypes

### Register New Type


### Associate Settings


## Syntax Association

### Built-in

File types automatically get syntax highlighting.

### Custom


## LSP Association


## Tips

1. Check filetype with `:set ft?`
2. Override with modeline if needed
3. Add custom extensions
4. Associate with existing syntax
