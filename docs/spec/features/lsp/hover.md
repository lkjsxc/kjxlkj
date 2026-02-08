# Hover Documentation

View documentation and type information for symbols.

## Keybindings (normative)

| Key | Action |
|---|---|
| `K` | Show hover popup for symbol under cursor |
| `gh` | Show hover (alternative) |

## Display Content

The hover popup shows information from the LSP server:

| Content Type | Includes |
|---|---|
| Function | Signature, parameter docs, return type, doc comment |
| Variable | Type, doc comment |
| Type | Full definition, doc comment |
| Keyword | Language reference (if server provides) |

## Popup Navigation

| Key | Action |
|---|---|
| `j` / `k` | Scroll down/up within popup |
| `<C-d>` / `<C-u>` | Page down/up |
| `<Esc>` | Close popup |

## Markdown Rendering

Hover content is rendered as Markdown. Supported elements: headers, bold/italic, inline code, code blocks (syntax-highlighted), lists, links (display only).

## Multiple Hovers

When multiple hover results exist at the same position:

| Key | Action |
|---|---|
| `]h` | Next hover result |
| `[h` | Previous hover result |

## Configuration

| Option | Default | Description |
|---|---|---|
| `hover` | `true` | Enable hover |
| `hover_delay` | `0` | Delay before showing (ms, 0 = instant on keypress) |

## LSP Fallback

Without LSP, hover falls back to showing the tree-sitter node type and any available type information.

## Related

- Signature help: [/docs/spec/features/lsp/signature-help.md](/docs/spec/features/lsp/signature-help.md)
- Completion: [/docs/spec/features/lsp/completion.md](/docs/spec/features/lsp/completion.md)
