# Editing Helpers

Built-in autopairs, surround, and comment features.

## Overview

These features replace common Vim plugins (auto-pairs,
vim-surround, vim-commentary) with built-in
equivalents sharing a unified configuration system.

## Auto-Pairs

Automatic bracket and quote pairing.

### Behavior

When typing an opening delimiter, the closing
delimiter is automatically inserted after the cursor.

### Default Pairs

| Open | Close | Context |
|------|-------|---------|
| `(` | `)` | All filetypes |
| `[` | `]` | All filetypes |
| `{` | `}` | All filetypes |
| `"` | `"` | All filetypes |
| `'` | `'` | All filetypes |
| `` ` `` | `` ` `` | Markdown, shell |
| `<` | `>` | HTML, XML, JSX |

### Smart Behaviors

| Action | Description |
|--------|-------------|
| Skip close | Typing `)` when next char is `)` moves past it |
| Delete pair | Backspace between empty pair deletes both |
| Newline pair | Enter between `{}` creates indented block |
| Wrap selection | Typing `(` with selection wraps it |

### Context Awareness

Auto-pairs are suppressed:
- Inside string literals (no double-quoting)
- Inside comments
- After escape character `\`
- When next character is a word character

## Surround

Surround text with delimiter pairs.

### Normal Mode Operations

| Sequence | Effect |
|----------|--------|
| `ys{motion}{char}` | Surround motion with char |
| `yss{char}` | Surround entire line |
| `cs{old}{new}` | Change surrounding delimiter |
| `ds{char}` | Delete surrounding delimiter |
| `yS{motion}{char}` | Surround on new lines |

### Examples

| Before | Sequence | After |
|--------|----------|-------|
| `hello` | `ysiw"` | `"hello"` |
| `"hello"` | `cs"'` | `'hello'` |
| `(hello)` | `ds(` | `hello` |
| `hello` | `ysiw<div>` | `<div>hello</div>` |
| `"hello"` | `cs"<p>` | `<p>hello</p>` |

### Tag Surround

When the surround character is `<`, the user is
prompted for a tag name. The opening and closing
tags are generated automatically.

### Visual Mode

In visual mode, `S{char}` surrounds the selection.

## Comment Toggle

Quick comment/uncomment operations.

### Normal Mode

| Sequence | Effect |
|----------|--------|
| `gcc` | Toggle comment on current line |
| `gc{motion}` | Toggle comment over motion range |
| `gC` | Toggle block comment |

### Visual Mode

| Sequence | Effect |
|----------|--------|
| `gc` | Toggle line comment on selection |
| `gC` | Toggle block comment on selection |

### Comment Strings

Comment strings are determined by filetype:

| Filetype | Line comment | Block comment |
|----------|-------------|---------------|
| Rust | `//` | `/* */` |
| Python | `#` | (none) |
| HTML | (none) | `<!-- -->` |
| Lua | `--` | `--[[ ]]` |
| C/C++ | `//` | `/* */` |

### Behavior Rules

- Uncomment if all lines in range are commented
- Comment if any line is uncommented
- Preserve indentation level
- Handle blank lines within range

## Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `autopairs.enabled` | bool | true | Enable auto-pairs |
| `autopairs.pairs` | list | (defaults) | Pair definitions |
| `surround.enabled` | bool | true | Enable surround |
| `comment.padding` | bool | true | Space after comment marker |
| `comment.sticky_cursor` | bool | true | Keep cursor position |

## Related

- Auto-pairs detail: [docs/spec/features/editing/auto-pairs.md](/docs/spec/features/editing/auto-pairs.md)
- Text objects: [docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
