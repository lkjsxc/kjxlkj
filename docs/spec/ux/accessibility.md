# Accessibility

kjxlkj strives to be accessible to all users.

## Screen Reader Support

Integration with terminal screen readers.

### Terminal Compatibility

For screen reader users:
- Use terminal with accessibility support
- Windows: Windows Terminal with Narrator
- macOS: Terminal.app with VoiceOver
- Linux: Accessible terminals vary

### Announcements

When available, kjxlkj announces:
- Mode changes ("INSERT", "NORMAL")
- Errors and warnings
- Search results
- File operations

## High Contrast Themes

Themes designed for maximum readability.

### Built-in Themes

| Theme | Background | Description |
|-------|------------|-------------|
| `high-contrast` | Dark | White text on black, bright accents |
| `high-contrast-light` | Light | Black text on white, bold accents |
| `accessible-dark` | Dark | WCAG AAA contrast ratios throughout |
| `accessible-light` | Light | WCAG AAA contrast ratios throughout |

Options:
- `high-contrast` - Maximum contrast
- `high-contrast-light` - Light background variant

Activate with: `set theme=high-contrast`

### Custom Colors

Override any highlight group to suit specific visual needs:

```
[highlight.Normal]
fg = "#FFFFFF"
bg = "#000000"

[highlight.Comment]
fg = "#FFD700"
style = "italic"

[highlight.Error]
fg = "#FF0000"
style = "bold,underline"
```

See `/home/lkjsxc/repos/kjxlkj/docs/spec/ui/themes.md` for the
full highlight group reference.

## Reduced Motion

Disable animations:

```
set animation=none
```

| Setting | Values | Default |
|---------|--------|---------|
| `animation` | `full`, `reduced`, `none` | `full` |
| `cursor.blink` | `true`, `false` | `true` |
| `scroll.smooth` | `true`, `false` | `true` |

When `animation=none`, all transitions are instant.

## Font Size / Zoom

Configure in your terminal emulator:
- kjxlkj uses terminal's font settings
- Ctrl++ / Ctrl+- typically zoom in terminals

## Keyboard-Only Navigation

kjxlkj is fully keyboard-driven:
- No mouse required
- All features accessible via keybindings
- Command palette for discoverability

### Navigation Without Arrow Keys

For users who can't use arrow keys:
- `h j k l` for movement
- `w b` for word movement
- `{ }` for paragraph movement
- `/` for search-based navigation

## Color Blindness

Adaptations for color-blind users.

### Deuteranopia/Protanopia Friendly

Avoid relying solely on red/green distinction. kjxlkj uses
alternative color pairs in colorblind-safe themes:

| Purpose | Avoid | Use Instead |
|---------|-------|-------------|
| Error vs. OK | Red vs. Green | Red vs. Blue |
| Diff add/remove | Green vs. Red | Blue vs. Orange |
| Git modified | Green highlight | Underline + blue |
| Warnings | Orange vs. red | Yellow + bold vs. red + underline |

Activate with: `set theme=high-contrast { colorblind = "deuteranopia" }`

Uses shapes and patterns, not just color:
- Error: X marker + red
- Warning: ! marker + yellow
- Info: i marker + blue

### Semantic Styling

Beyond color, kjxlkj uses:
- Bold for emphasis
- Underline for links
- Italics for comments
- Distinct shapes in gutter

## Large Cursor

Configure cursor appearance for visibility:

| Setting | Values | Default |
|---------|--------|---------|
| `cursor.shape` | `block`, `line`, `underline` | `block` |
| `cursor.blink` | `true`, `false` | `true` |
| `cursor.blink_rate` | Milliseconds | `530` |
| `cursor.highlight_line` | `true`, `false` | `false` |
| `cursor.highlight_column` | `true`, `false` | `false` |

```
set cursor.shape=block
set cursor.blink=true
set cursor.blink_rate=400
set cursor.highlight_line=true
```

Enabling `highlight_line` draws a visible bar across the full
width of the viewport, making cursor location easy to find.

## Sticky Keys Support

kjxlkj works with OS sticky keys:
- Ctrl+key combinations work with sticky Ctrl
- Shift+key combinations work with sticky Shift

kjxlkj receives modifier+key as a single event from the terminal,
so OS-level sticky keys are fully transparent. No editor
configuration is needed.

| OS | Enable Sticky Keys |
|----|-------------------|
| Windows | Settings > Accessibility > Keyboard |
| macOS | System Settings > Accessibility > Keyboard |
| Linux (GNOME) | Settings > Accessibility > Typing |
| Linux (X11) | `xkbset sticky -twokey` |

For multi-key sequences (e.g., `gc`), sticky keys are not
involved -- these are sequential keypresses, not held modifiers.

## Reporting Issues

For accessibility issues:
1. Open issue on GitHub
2. Tag with "accessibility" label
3. Describe assistive technology used
4. Include terminal and OS version
