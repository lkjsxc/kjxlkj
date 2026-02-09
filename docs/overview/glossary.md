# Glossary

Back: [/docs/overview/README.md](/docs/overview/README.md)

Shared terminology used across documentation.

## Usage Note

This glossary is explanatory and may use simplified definitions.

Normative behavior definitions live in `/docs/spec/`.
Current availability lives in `/docs/reference/`.

## Editor Concepts

| Term | Definition |
|---|---|
| Buffer | In-memory text container |
| Window | Viewport into a buffer |
| Mode | Input handling context |
| Motion | Cursor movement command |
| Operator | Text-changing action applied to a range |
| Text object | Semantic text region |
| Register | Named text storage |
| Mark | Saved cursor position |
| Jumplist | Navigation history |

## UI Concepts

| Term | Definition |
|---|---|
| View | Full-content UI component |
| Widget | Smaller UI element inside a view |
| Modal | Temporary overlay capturing focus |
| Panel | Layout container |
| Sidebar | Docked auxiliary panel |
| Gutter | Margin with metadata (line numbers, diagnostics) |
| Statusline | Bottom status surface |
| Command line | Ex-command input surface |

## Key Notation

| Notation | Meaning |
|---|---|
| `<leader>` | Leader key |
| `<C-x>` | Ctrl+x |
| `<A-x>` | Alt+x |
| `<Esc>` | Escape |
| `<CR>` | Enter |
| `<Tab>` | Tab |
| `<Space>` | Space |

## Technical Terms

| Term | Definition |
|---|---|
| Rope | Tree-oriented text storage structure |
| Viewport | Visible portion of a buffer |
| LSP | Language Server Protocol |
| Syntax tree | Parsed structure used by syntax features |
| Event loop | Core event-processing loop |

## File Concepts

| Term | Definition |
|---|---|
| Working directory | Base path for file operations |
| Current buffer | Active buffer |
| Alternate buffer | Previously active buffer |
| Modified | Buffer has unsaved changes |
| Read-only | Buffer should not be modified |
