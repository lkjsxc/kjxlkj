# Highlight Groups

Back: [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)

Named highlight groups define text appearance for syntax elements and UI components.

## Overview

Each highlight group specifies foreground color, background color, and style attributes (bold, italic, underline, etc.). Groups are linked to Tree-sitter capture names and UI elements.

## Standard Groups

| Group | Usage |
|---|---|
| `Normal` | Default text |
| `Comment` | Code comments |
| `String` | String literals |
| `Number` | Numeric literals |
| `Keyword` | Language keywords |
| `Function` | Function names |
| `Type` | Type names |
| `Identifier` | Variables and identifiers |
| `Operator` | Operators |
| `Constant` | Constants |

## UI Groups

| Group | Usage |
|---|---|
| `StatusLine` | Active statusline |
| `StatusLineNC` | Inactive statusline |
| `LineNr` | Line numbers |
| `CursorLine` | Current line highlight |
| `CursorLineNr` | Current line number |
| `Visual` | Visual selection |
| `Search` | Search matches |
| `IncSearch` | Incremental search match |
| `Pmenu` | Popup menu |
| `PmenuSel` | Selected popup menu item |

## Attributes

| Attribute | Description |
|---|---|
| `fg` | Foreground color (hex or named) |
| `bg` | Background color |
| `bold` | Bold text |
| `italic` | Italic text |
| `underline` | Underline |
| `undercurl` | Wavy underline |
| `strikethrough` | Strikethrough |
| `reverse` | Swap fg and bg |

## Linking

A highlight group can be linked to another group:

`:highlight link {from} {to}` — `{from}` inherits the style of `{to}`.

## Tree-sitter Mapping

Tree-sitter capture names map to highlight groups:

| Capture | Group |
|---|---|
| `@keyword` | `Keyword` |
| `@function` | `Function` |
| `@string` | `String` |
| `@comment` | `Comment` |
| `@type` | `Type` |
| `@variable` | `Identifier` |

## Related

- Syntax highlighting: [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)
- Colorscheme: [/docs/spec/features/syntax/colorscheme-creation.md](/docs/spec/features/syntax/colorscheme-creation.md)
- Semantic tokens: [/docs/spec/features/syntax/semantic-tokens.md](/docs/spec/features/syntax/semantic-tokens.md)
