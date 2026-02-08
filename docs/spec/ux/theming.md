# Theming

Back: [/docs/spec/ux/README.md](/docs/spec/ux/README.md)

Theming is pure data applied at render time. Theme changes NEVER mutate core editor state.

## Theme resolution (normative)

1. On startup, load the theme specified by the `colorscheme` option.
2. The theme is a map from highlight group names to style definitions.
3. The render task uses this map to assign colors/styles to each cell in the cell grid.
4. Changing the theme (`:colorscheme {name}`) replaces the map and triggers a full redraw.

## Highlight group catalog (normative)

| Group | Applied to |
|---|---|
| `Normal` | Default text and background |
| `NormalFloat` | Floating windows |
| `Cursor` | Character under cursor in Normal mode |
| `CursorLine` | Current line highlight |
| `CursorColumn` | Current column highlight |
| `LineNr` | Line numbers in gutter |
| `CursorLineNr` | Current line number |
| `SignColumn` | Sign column background |
| `StatusLine` | Active window statusline |
| `StatusLineNC` | Inactive window statusline |
| `TabLine` | Inactive tab labels |
| `TabLineSel` | Active tab label |
| `TabLineFill` | Tab line background fill |
| `WinSeparator` | Window separator characters |
| `Visual` | Visual selection highlight |
| `Search` | Active search match highlight |
| `IncSearch` | Incremental search highlight |
| `MatchParen` | Matching bracket highlight |
| `Pmenu` | Completion popup normal |
| `PmenuSel` | Completion popup selected item |
| `PmenuSbar` | Popup scrollbar background |
| `PmenuThumb` | Popup scrollbar thumb |
| `Folded` | Folded line indicator |
| `FoldColumn` | Fold column |
| `DiffAdd` | Added lines in diff |
| `DiffChange` | Changed lines in diff |
| `DiffDelete` | Deleted lines in diff |
| `DiffText` | Changed text within a changed line |
| `DiagnosticError` | Error diagnostics |
| `DiagnosticWarn` | Warning diagnostics |
| `DiagnosticInfo` | Info diagnostics |
| `DiagnosticHint` | Hint diagnostics |
| `Comment` | Syntax: comments |
| `String` | Syntax: string literals |
| `Number` | Syntax: numeric literals |
| `Keyword` | Syntax: language keywords |
| `Function` | Syntax: function names |
| `Type` | Syntax: type names |
| `Identifier` | Syntax: identifiers |
| `Operator` | Syntax: operators |
| `Constant` | Syntax: constants |
| `Special` | Syntax: special characters |
| `Error` | Syntax: error tokens |
| `Todo` | Syntax: TODO/FIXME annotations |

## Style definition (normative)

Each highlight group maps to a style with these optional fields:

| Field | Type | Description |
|---|---|---|
| `fg` | Color | Foreground color |
| `bg` | Color | Background color |
| `bold` | bool | Bold attribute |
| `italic` | bool | Italic attribute |
| `underline` | bool | Underline attribute |
| `strikethrough` | bool | Strikethrough attribute |
| `reverse` | bool | Swap fg and bg |
| `link` | string | Inherit from another group (e.g., `Comment` links to `Special`) |

Color values MUST be specified as RGB hex (`#RRGGBB`). The render pipeline maps these to the terminal's color capability (true color, 256, or 16).

## Built-in themes (normative)

The editor MUST ship with at least two built-in themes:

| Theme name | Description |
|---|---|
| `default` | Dark theme suitable for most terminals |
| `light` | Light theme for light terminal backgrounds |

## Related

- Theme file format: [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)
- Syntax highlighting: [/docs/spec/features/syntax/syntax.md](/docs/spec/features/syntax/syntax.md)
- Render pipeline: [/docs/spec/architecture/render-pipeline.md](/docs/spec/architecture/render-pipeline.md)
