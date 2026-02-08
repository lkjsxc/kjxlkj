# Filetype Configuration

Per-filetype settings and behaviors.

## Overview

Configure settings that apply only when editing
specific file types. Filetype detection runs on
buffer open and sets the `filetype` option, which
triggers loading of filetype-specific configuration.

## Configuration Location

Filetype configs are stored in one of two places:
1. Inline in `~/.config/kjxlkj/config.toml` under `[languages.{lang}]`
2. Dedicated files at `~/.config/kjxlkj/languages/{lang}.toml`

Dedicated files take precedence over inline config.

## In Main Config

Under `[languages.rust]` (or any language name):
- `tab_width = 4`
- `indent_unit = "    "` (4 spaces)
- `rulers = [100]`
- `format_on_save = true`

## Dedicated Files

### rust.toml

A file at `~/.config/kjxlkj/languages/rust.toml`
with top-level keys (no `[languages.rust]` wrapper needed):
- `tab_width = 4`
- `formatter = { command = "rustfmt", args = ["--edition", "2021"] }`
- `lsp = { command = "rust-analyzer" }`

## Available Settings

### Editor

| Key | Type | Description |
|-----|------|-------------|
| `tab_width` | integer | Display width of tab character |
| `indent_unit` | string | String inserted per indent level |
| `rulers` | array | Column positions for ruler lines |
| `line_ending` | string | `"lf"` or `"crlf"` |
| `trim_trailing_whitespace` | bool | Remove trailing spaces on save |
| `insert_final_newline` | bool | Ensure file ends with newline |
| `max_line_length` | integer | Soft wrap / highlight column |

### Formatting

| Key | Type | Description |
|-----|------|-------------|
| `formatter` | table | External formatter command |
| `format_on_save` | bool | Auto-format before write |
| `format_timeout` | integer | Formatter timeout in ms |

### Comments

| Key | Type | Description |
|-----|------|-------------|
| `comment_line` | string | Line comment prefix (e.g. `"//"`) |
| `comment_block_start` | string | Block comment open (e.g. `"/*"`) |
| `comment_block_end` | string | Block comment close (e.g. `"*/"`) |

### Indentation

| Key | Type | Description |
|-----|------|-------------|
| `auto_indent` | bool | Copy indent from previous line |
| `smart_indent` | bool | Language-aware indent adjustments |
| `indent_keys` | string | Keys that trigger re-indent |

## Language Specific

### Rust

`tab_width = 4`, `indent_unit = "    "`, `rulers = [100]`,
`comment_line = "//"`, `format_on_save = true`.

### Python

`tab_width = 4`, `indent_unit = "    "`, `rulers = [79, 120]`,
`comment_line = "#"`, `trim_trailing_whitespace = true`.

### JavaScript

`tab_width = 2`, `indent_unit = "  "`, `rulers = [80]`,
`comment_line = "//"`.

### Markdown

`tab_width = 4`, `indent_unit = "    "`,
`trim_trailing_whitespace = false` (trailing spaces meaningful),
`insert_final_newline = true`.

## LSP Configuration

### Per-Language Server

Under `[languages.{lang}.lsp]`:
- `command` (string): LSP server binary
- `args` (array): Command-line arguments
- `config` (table): Server-specific initialization options
- `root_markers` (array): Files indicating project root

## Keybindings

### Per-Filetype

Under `[languages.{lang}.keys.normal]` (or `insert`, `visual`):
filetype-specific key overrides that apply only in
buffers of that filetype.

## Abbreviations

Under `[languages.{lang}.abbreviations]`:
key-value pairs where the key is the abbreviation trigger
and the value is the expansion text.

## Snippets

Under `[languages.{lang}.snippets]`:
snippet definitions with prefix, body, and description.
Snippets are also loaded from `snippets/{lang}.toml`.

## Auto Commands

Filetype configuration supports `auto_commands` (array of tables)
with `event`, `pattern`, and `command` fields for per-filetype
autocommand hooks.

## Tips

1. Use dedicated files for complex config
2. Keep global defaults minimal
3. Override per-project with `.kjxlkj.toml`
4. Test with `:set option?`
