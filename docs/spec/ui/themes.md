# Theme System

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

kjxlkj includes multiple built-in themes and supports user-defined themes.

## Built-in themes

| Theme | Variant | Description |
|---|---|---|
| Default Dark | dark | Modern dark theme with balanced contrast |
| Catppuccin Mocha | dark | Warm, pastel colors |
| Tokyo Night | dark | Blue-tinted dark theme |
| Dracula | dark | Classic purple-tinted dark theme |
| Nord | dark | Arctic, bluish color palette |
| Gruvbox Dark | dark | Retro earthy colors with high contrast |
| One Dark | dark | Atom-inspired balanced dark theme |
| Default Light | light | Clean light theme for daytime use |
| Catppuccin Latte | light | Warm pastel light theme |
| Gruvbox Light | light | Warm, retro light theme |
| Solarized Light | light | Precision colors for visibility |
| High Contrast Dark | dark | Maximum contrast for accessibility |
| High Contrast Light | light | Light variant with maximum contrast |

## Theme file format

Themes are defined in TOML files stored under `~/.config/kjxlkj/themes/`.

### Meta section

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | string | yes | Theme display name |
| `variant` | `"dark"` or `"light"` | yes | Base variant for fallback behavior |
| `inherits` | string | no | Name of base theme to extend |

### Palette section

| Key | Type | Description |
|---|---|---|
| `bg` | hex color string | Background color (e.g. `"#1a1b26"`) |
| `fg` | hex color string | Foreground color (e.g. `"#c0caf5"`) |
| `red` | hex color string | Red palette slot |
| `green` | hex color string | Green palette slot |
| `blue` | hex color string | Blue palette slot |
| `yellow` | hex color string | Yellow palette slot |

### Core highlight groups

| Group | Purpose | Supported fields |
|---|---|---|
| `Normal` | Default text | `fg`, `bg` |
| `Comment` | Code comments | `fg`, `style` |
| `Keyword` | Language keywords | `fg`, `style` |
| `String` | String literals | `fg` |
| `Function` | Function names | `fg`, `style` |
| `Type` | Type names | `fg` |
| `Error` | Error highlights | `fg`, `bg`, `style` |
| `CursorLine` | Current line | `bg` |
| `StatusLine` | Status bar | `fg`, `bg`, `style` |
| `LineNr` | Line numbers | `fg` |

Style values: `bold`, `italic`, `underline`, `strikethrough`, or comma-separated combinations.

### Inheritance

When `inherits` is set, groups not listed are inherited from the base theme. Chains are resolved at load time (no runtime cost).

## Terminal color detection (normative)

| Setting | Behavior |
|---|---|
| `auto` | Detect from `$COLORTERM` and terminfo |
| `truecolor` | Force 24-bit color output |
| `256` | Use 256-color palette (closest match) |
| `16` | Use terminal's 16 ANSI colors only |

| Condition | Result |
|---|---|
| `$COLORTERM` = `truecolor` or `24bit` | True color |
| terminfo `colors >= 256` | 256-color mode |
| Otherwise | 16-color fallback |

In 16-color mode, kjxlkj maps theme colors to the terminal's ANSI palette. Themes MAY provide explicit 16-color overrides via `palette.ansi16` mapping color names to ANSI indices (`0`-`15`).

## Related

- UI index: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)
- Theming UX: [/docs/spec/ux/theming.md](/docs/spec/ux/theming.md)
- Highlight groups: [/docs/spec/features/syntax/highlight-groups.md](/docs/spec/features/syntax/highlight-groups.md)
