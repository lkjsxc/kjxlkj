# Inlay Hints

Inline type and parameter annotations powered by LSP.

## Overview

Inlay hints display non-editable annotations inline within the code — type hints on variables, parameter names at call sites, and chaining hints on method chains. They are rendered in a distinct dimmed style to avoid confusion with actual code.

## Types of Hints (normative)

| Hint Type | Display | Example |
|---|---|---|
| Type hint | After variable name | `let x` → `let x: i32` |
| Parameter hint | Before argument | `foo(` → `foo(name:` |
| Chaining hint | After method chain | `.map(...)` → `.map(...): Vec<i32>` |

## Keybindings (normative)

| Key | Action |
|---|---|
| `<leader>ih` | Toggle inlay hints on/off |

## Configuration

| Option | Default | Description |
|---|---|---|
| `inlay_hints` | `true` | Enable inlay hints |
| `inlay_hints_type` | `true` | Show type hints |
| `inlay_hints_parameter` | `true` | Show parameter name hints |
| `inlay_hints_chaining` | `true` | Show chaining type hints |
| `inlay_hints_delay` | `200` | Delay in ms before requesting hints |

## Styling

Hints are rendered with a distinct face (typically dimmed foreground, no background) so they are visually distinct from actual code. The face is configurable via theme `inlay_hint` highlight group.

## Performance

Hints are requested only for the visible viewport. On scroll, new hints are fetched. The `inlay_hints_delay` debounce prevents excessive LSP requests during rapid scrolling.

## LSP Server Support

| Server | Type | Parameter | Chaining |
|---|---|---|---|
| rust-analyzer | Yes | Yes | Yes |
| typescript-language-server | Yes | Yes | No |
| clangd | Yes | Yes | No |
| gopls | Yes | Yes | No |

## Interactive Behavior

Hovering on a hint (mouse or cursor) shows the full type if it was truncated. The hint text is not editable and does not affect cursor positioning.

## Related

- Syntax highlighting: [/docs/spec/features/syntax/syntax-files.md](/docs/spec/features/syntax/syntax-files.md)
- Semantic tokens: [/docs/spec/features/syntax/semantic-tokens.md](/docs/spec/features/syntax/semantic-tokens.md)
