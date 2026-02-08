# Theme Gallery

kjxlkj includes multiple built-in themes.

## Dark Themes

### Default Dark
Modern dark theme with balanced contrast.

### Catppuccin Mocha
Warm, pastel colors. Community favorite.

### Tokyo Night
Blue-tinted dark theme inspired by Tokyo cityscape.

### Dracula
Classic purple-tinted dark theme.

### Nord
Arctic, bluish color palette.

### Gruvbox Dark
Retro earthy colors with high contrast.

### One Dark
Atom-inspired balanced dark theme.

## Light Themes

### Default Light
Clean light theme for daytime use.

### Catppuccin Latte
Warm pastel light theme.

### Gruvbox Light
Warm, retro light theme.

### Solarized Light
Precision colors for visibility.

## High Contrast

### High Contrast Dark
Maximum contrast for accessibility.

### High Contrast Light
Light variant with maximum contrast.

## Custom Themes

### Full Customization

Define a complete theme by specifying every highlight group.
Create a file under the themes directory and set each group:

```toml
[meta]
name = "my-theme"
variant = "dark"           # "dark" or "light"

[palette]
bg       = "#1a1b26"
fg       = "#c0caf5"
red      = "#f7768e"
green    = "#9ece6a"
blue     = "#7aa2f7"
yellow   = "#e0af68"
```

Core highlight groups:

| Group | Purpose | Fields |
|-------|---------|--------|
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

Style values: `bold`, `italic`, `underline`, `strikethrough`,
or comma-separated combinations like `bold,italic`.

### Inherit and Override

Extend a built-in theme and override only what you need:

```toml
[meta]
name = "my-variant"
inherits = "catppuccin-mocha"

[highlight.Comment]
fg = "#888888"
style = "italic"

[highlight.Keyword]
style = "bold"
```

All groups not listed are inherited from the base theme.
Inheritance chains are resolved at load time (no runtime cost).

## Terminal Integration

Theme colors work best when terminal supports:
- True color (24-bit)
- Or 256 color mode

Check support:

```
set termcolors=auto
```

| Setting | Behavior |
|---------|----------|
| `auto` | Detect from `$COLORTERM` and terminfo |
| `truecolor` | Force 24-bit color output |
| `256` | Use 256-color palette (closest match) |
| `16` | Use terminal's 16 ANSI colors only |

Detection logic:

| Condition | Result |
|-----------|--------|
| `$COLORTERM` = `truecolor` or `24bit` | True color |
| terminfo `colors >= 256` | 256-color mode |
| Otherwise | 16-color fallback |

In 16-color mode, kjxlkj maps theme colors to the terminal's
ANSI palette. The appearance depends on the terminal's own
color scheme. Themes can provide explicit 16-color overrides:

```toml
[palette.ansi16]
red    = "1"     # ANSI color index
green  = "2"
yellow = "3"
blue   = "4"
```
