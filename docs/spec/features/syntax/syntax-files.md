# Syntax Files

Configure syntax highlighting.

## Overview

kjxlkj uses tree-sitter for syntax highlighting
with configurable queries and themes.

## Built-in Languages

### Fully Supported

- Rust
- Python
- JavaScript/TypeScript
- Go
- C/C++
- Markdown
- JSON/YAML/TOML
- HTML/CSS
- Bash
- Lua

### Partial Support

Additional languages via tree-sitter.

## Configuration

### Enable/Disable


## Tree-sitter Queries

### Location


### Query Format


## Highlight Groups

### Standard Groups

| Group | Description |
|-------|-------------|
| `@comment` | Comments |
| `@string` | String literals |
| `@function` | Functions |
| `@keyword` | Keywords |
| `@type` | Types |
| `@variable` | Variables |
| `@constant` | Constants |
| `@operator` | Operators |

### Custom Groups


## Theme Integration

### Mapping


## Custom Languages

### Register Language


### Parser Installation

Tree-sitter parsers compiled as shared objects.

## Injection

### Embedded Languages


### Common Injections

- SQL in strings
- Regex in strings
- Markdown in comments

## Performance

### Incremental Parsing

Only changed regions re-parsed.

### Viewport Priority

Visible lines parsed first.

## Debugging

### Show Syntax Tree


### Show Highlight


Shows capture groups at cursor.

## Tips

1. Use built-in languages when possible
2. Customize colors via theme
3. Check `:Inspect` for highlighting
4. Report missing captures

## Commands

