# Comment Toggling

kjxlkj provides easy comment toggling.

## Overview

Toggle comments with a single keypress:

| Key | Action |
|-----|--------|
| `gcc` | Toggle line comment on current line |
| `gc{motion}` | Toggle line comment over motion |
| `gbc` | Toggle block comment on current line |
| `gb{motion}` | Toggle block comment over motion |

## Configuration

Comment strings are configured per filetype in `config.toml` or
per-filetype config files under `ftconfig/`. The `commentstring`
option controls line comments; `block_commentstring` controls block
comments. Both are set automatically when a filetype is detected.

```toml
[filetype.rust]
commentstring = "//"
block_commentstring = ["/*", "*/"]

[filetype.python]
commentstring = "#"
```

## Keybindings

### Line Comment

| Key | Mode | Action |
|-----|------|--------|
| `gcc` | Normal | Toggle comment on current line |
| `gc` | Visual | Toggle comment on selected lines |
| `{count}gcc` | Normal | Toggle comment on {count} lines |

`gcc` prepends the filetype `commentstring` to the line. If all
targeted lines already start with the comment marker, it removes
the markers instead (uncomment).

### Block Comment

| Key | Mode | Action |
|-----|------|--------|
| `gbc` | Normal | Toggle block comment on current line |
| `gb` | Visual | Wrap selection in block comment |

`gbc` wraps the current line content in block comment delimiters.
If the line is already block-commented, the delimiters are removed.

### With Motion

`gc` acts as an operator and accepts any motion:

| Key | Action |
|-----|--------|
| `gcip` | Comment inner paragraph |
| `gc3j` | Comment current line and 3 lines down |
| `gcG` | Comment from cursor to end of file |
| `gcap` | Comment around paragraph |
| `gciB` | Comment inner braces block |

Examples:
- `gcip` - Comment inner paragraph
- `gc3j` - Comment 3 lines down
- `gcG` - Comment to end of file

### Visual Mode

Select lines with `V` (linewise) or `v` (characterwise), then:

| Key | Action |
|-----|--------|
| `gc` | Toggle line comment on selected lines |
| `gb` | Wrap selection in block comment delimiters |

For characterwise selections, `gc` comments entire lines that the
selection spans. `gb` inserts block delimiters at exact selection
boundaries.

## Language Support

Auto-detects comment style per language:

| Language | Line | Block |
|----------|------|-------|
| Rust | `//` | `/* */` |
| Python | `#` | `""" """` |
| HTML | N/A | `<!-- -->` |
| CSS | N/A | `/* */` |
| Lua | `--` | `--[[ ]]` |
| SQL | `--` | `/* */` |

## Custom Comment Styles

Override for specific languages:

```toml
[filetype.custom_lang]
commentstring = ";;"
block_commentstring = ["{-", "-}"]
```

When `commentstring` is empty, `gcc` is a no-op for that filetype.
When `block_commentstring` is absent, `gbc` falls back to using
`commentstring` on each line.

## Smart Comments

### Uncomment Detection

Automatically detects if lines are commented:
- All lines commented --> uncomment
- Mixed --> comment all
- None commented --> comment all

### Padding

A single space is inserted between the comment marker and code by
default. Controlled by the `comment_padding` boolean option
(default: `true`).

Without padding: `//code`
With padding: `// code`

When uncommenting, padding spaces are removed if they match the
expected pattern. Trailing whitespace is not added to blank lines.

## Block Comments

### Toggle Block

`gbc` toggles block comment on current line.

### Wrap Selection

In visual mode, `gb` wraps selection:

The opening delimiter is placed at the start of the selection,
the closing delimiter at the end. For multi-line selections, the
delimiters appear on their own lines if `block_comment_own_line`
is `true` (default: `false`).

## Advanced

### Comment Above/Below

| Key | Action |
|-----|--------|
| `gcO` | Add comment line above, enter Insert mode |
| `gco` | Add comment line below, enter Insert mode |

Adds comment line and enters insert mode.

### TODO Comments

Quick TODO insertion:

| Key | Inserts |
|-----|---------|
| `gcO` then type | `// TODO: ` (cursor after colon) |

The following keyword patterns are highlighted inside comments
using the `@comment.todo` highlight group family:

| Pattern | Highlight Group | Default Color |
|---------|----------------|---------------|
| `TODO` | `@comment.todo` | Yellow bold |
| `FIXME` | `@comment.fixme` | Red bold |
| `NOTE` | `@comment.note` | Blue bold |
| `HACK` | `@comment.hack` | Orange bold |
| `WARN` | `@comment.warning` | Orange bold |
| `PERF` | `@comment.perf` | Purple bold |

Patterns are matched case-sensitively, followed by a colon or
end of line (e.g., `// TODO: fix this` or `// TODO`).

## Tree-sitter Integration

Uses tree-sitter for:
- Accurate language detection in mixed files
- Respect for string literals (don't comment inside)
- JSX/TSX component handling
