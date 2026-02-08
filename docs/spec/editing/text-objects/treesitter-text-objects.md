# Tree-sitter Text Objects

Back: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)

Text objects defined by Tree-sitter syntax nodes.

## Overview

Tree-sitter text objects use the parsed syntax tree to define structural selections. They are language-aware and operate on functions, classes, parameters, arguments, etc.

## Built-in Tree-sitter Objects

| Text Object | Description |
|---|---|
| `af` / `if` | Around/inner function |
| `ac` / `ic` | Around/inner class |
| `aa` / `ia` | Around/inner argument/parameter |
| `aC` / `iC` | Around/inner comment |
| `al` / `il` | Around/inner loop |
| `ai` / `ii` | Around/inner conditional (if) |

## Inner vs Around

| Object | Selects |
|---|---|
| `if` | Function body (excluding signature and braces) |
| `af` | Entire function (including signature, braces, decorators) |
| `ia` | Single argument value |
| `aa` | Argument including trailing comma/separator |

## Navigation

| Key | Description |
|---|---|
| `]f` | Jump to next function start |
| `[f` | Jump to previous function start |
| `]c` | Jump to next class start |
| `[c` | Jump to previous class start |

## Language Support

Tree-sitter text objects require query files for each language. These define which syntax nodes map to which text object types.

| Language | Function Node | Class Node |
|---|---|---|
| Rust | `function_item` | `struct_item`, `impl_item` |
| Python | `function_definition` | `class_definition` |
| JavaScript | `function_declaration`, `arrow_function` | `class_declaration` |

## Fallback

If no Tree-sitter parser is available for the current file type, tree-sitter text objects are not available. The editor emits a message.

## Related

- Text objects: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
- Tree-sitter: [/docs/spec/editing/text-objects/treesitter-text-objects.md](/docs/spec/editing/text-objects/treesitter-text-objects.md)
