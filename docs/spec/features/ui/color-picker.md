# Color Picker

Preview and edit colors inline.

## Color Preview (normative)

Colors in source code are detected and shown with a small colored square inline before the color value. Detected formats:

| Format | Example |
|---|---|
| Hex | `#ff5370`, `#f53` |
| RGB | `rgb(255, 83, 112)` |
| RGBA | `rgba(255, 83, 112, 0.5)` |
| HSL | `hsl(348, 100%, 66%)` |
| Named CSS | `red`, `blue`, `tomato` |

## Color Picker Interface

| Key | Action |
|---|---|
| `<leader>cp` | Open color picker at cursor |

### Picker Controls

| Key | Action |
|---|---|
| `h` / `l` | Decrease / increase current component |
| `j` / `k` | Next / previous component (R/G/B or H/S/L) |
| `<CR>` | Apply selected color |
| `<Esc>` | Cancel |

## Format Conversion

The picker can convert between formats. Select the target format before applying.

## Configuration

| Option | Default | Description |
|---|---|---|
| `color_preview` | `true` | Show inline color squares |
| `color_picker` | `true` | Enable `<leader>cp` |

## Supported File Types

Color preview is enabled by default in CSS, SCSS, Less, HTML, JavaScript, TypeScript, and TOML/YAML config files. Other file types can be added via configuration.

## Related

- UI overview: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)
