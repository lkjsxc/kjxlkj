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

## Language detection (normative)

The syntax engine MUST choose a language id deterministically for each buffer.

Minimum detection strategy (in priority order):

1. An explicit per-buffer override (if the implementation supports it).
2. A file-extension mapping.
3. Fallback to `plain` (no structured highlighting).

The built-in extension mapping MUST include at least:

| Extension | Language id |
|---|---|
| `.rs` | `rust` |
| `.py` | `python` |
| `.js`, `.jsx`, `.ts`, `.tsx` | `javascript` / `typescript` (implementation-defined split) |
| `.go` | `go` |
| `.c`, `.h` | `c` |
| `.cpp`, `.cc`, `.cxx`, `.hpp`, `.hh`, `.hxx` | `cpp` |
| `.md` | `markdown` |
| `.json` | `json` |
| `.yaml`, `.yml` | `yaml` |
| `.toml` | `toml` |
| `.html`, `.htm` | `html` |
| `.css` | `css` |
| `.sh`, `.bash` | `bash` |
| `.lua` | `lua` |

For any language listed as “Fully Supported”, the implementation MUST ship a working parser + highlight query set such that a typical file renders with non-empty highlight spans by default.

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
