# Glossary

Terminology used throughout the documentation.

## Editor Concepts

| Term | Definition |
|------|------------|
| **Buffer** | In-memory text container backed by a rope data structure |
| **Window** | Viewport into a buffer; multiple windows can show the same buffer |
| **Mode** | Input handling context (Normal, Insert, Visual, Command, Replace) |
| **Motion** | Cursor movement command (e.g., `w`, `j`, `$`) |
| **Operator** | Action that operates on text (e.g., `d`, `y`, `c`) |
| **Text Object** | Semantic text region (e.g., `iw`, `i"`, `ap`) |
| **Register** | Named storage for text (clipboard, macros) |
| **Mark** | Saved cursor position |
| **Jumplist** | Navigation history for `Ctrl-o` / `Ctrl-i` |

## UI Concepts

| Term | Definition |
|------|------------|
| **View** | Full-content UI component occupying a panel |
| **Widget** | Smaller UI element within a view |
| **Modal** | Temporary overlay capturing focus |
| **Panel** | Layout container that can be split |
| **Sidebar** | Docked panel (left/right) for auxiliary views |
| **Gutter** | Side margin showing line numbers, diagnostics |
| **Status Bar** | Bottom line showing mode, file, position |
| **Command Line** | Bottom input for ex commands |

## Keybinding Notation

| Notation | Meaning |
|----------|---------|
| `<leader>` | Space key (configurable) |
| `<C-x>` | Ctrl+x |
| `<A-x>` | Alt+x |
| `<Esc>` | Escape key |
| `<CR>` | Enter/Return |
| `<Tab>` | Tab key |
| `<Space>` | Space bar |

## Technical Terms

| Term | Definition |
|------|------------|
| **Rope** | Tree-based text data structure for efficient editing |
| **Viewport** | Visible portion of a buffer |
| **Syntax Tree** | Parsed representation for highlighting |
| **LSP** | Language Server Protocol for IDE features |
| **Tree-sitter** | Incremental parsing library |
| **Screen Buffer** | Double-buffered terminal output |
| **Event Loop** | Main thread processing input and rendering |

## Mode-Specific Terms

| Term | Definition |
|------|------------|
| **Normal Mode** | Default mode for navigation and commands |
| **Insert Mode** | Mode for text insertion |
| **Visual Mode** | Mode for text selection (character/line/block) |
| **Command Mode** | Mode for ex commands (`:`) |
| **Replace Mode** | Mode for overwriting text |
| **Operator-Pending** | Transient state waiting for motion |

## File Concepts

| Term | Definition |
|------|------------|
| **Working Directory** | Project root for file operations |
| **Current Buffer** | Active buffer in current window |
| **Alternate Buffer** | Previously active buffer (`Ctrl-^`) |
| **Modified** | Buffer has unsaved changes |
| **Read-Only** | Buffer cannot be modified |
