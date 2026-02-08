# Colorscheme Creation

Back: [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)

Custom colorschemes are TOML files stored in `~/.config/kjxlkj/themes/`.

## File structure (normative)

A theme TOML file MUST contain:

| Top-level key | Type | Description |
|---|---|---|
| `name` | string | Display name of the theme |
| `background` | `"dark"` or `"light"` | Theme variant for `background` option |
| `[palette]` | table | Named color definitions (e.g., `red = "#e06c75"`) |
| `[highlights]` | table | Highlight group definitions |
| `[terminal_colors]` | table | 16 ANSI terminal color overrides |

## Highlight group definition (normative)

Each key under `[highlights]` is a highlight group name. The value is a table:

| Field | Type | Default | Description |
|---|---|---|---|
| `fg` | string (color or palette ref) | inherited | Foreground color |
| `bg` | string (color or palette ref) | inherited | Background color |
| `bold` | boolean | false | Bold attribute |
| `italic` | boolean | false | Italic attribute |
| `underline` | boolean | false | Underline attribute |
| `strikethrough` | boolean | false | Strikethrough attribute |
| `link` | string (group name) | none | Inherit from another group |

Color values can be hex (`"#rrggbb"`), palette references (`"palette.red"`), or named ANSI colors (`"red"`, `"blue"`).

## Required highlight groups

A theme MUST define at least these groups (or link them):

| Group | Used for |
|---|---|
| `Normal` | Default text |
| `Cursor` | Cursor cell |
| `Visual` | Visual selection |
| `Search` | Search match highlighting |
| `StatusLine` | Active status line |
| `StatusLineNC` | Inactive status line |
| `LineNr` | Line numbers |
| `CursorLineNr` | Current line number |
| `Comment` | Code comments |
| `String` | String literals |
| `Keyword` | Language keywords |
| `Function` | Function names |
| `Type` | Type names |
| `Error` | Error diagnostics |
| `Warning` | Warning diagnostics |

## Activation

Set the theme in config: `colorscheme = "theme_name"`. At runtime: `:colorscheme theme_name`.

## Related

- Theme system: [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)
- Highlight groups: [/docs/spec/features/syntax/highlight-groups.md](/docs/spec/features/syntax/highlight-groups.md)

