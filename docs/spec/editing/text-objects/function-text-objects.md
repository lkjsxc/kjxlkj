# Function Text Objects

Text objects for selecting function definitions and bodies.

## Commands (normative)

| Object | Description |
|---|---|
| `if` | Inner function: function body only (excluding signature and delimiters) |
| `af` | Around function: entire function (signature + body + closing) |

## Detection Method

Function boundaries are detected via tree-sitter AST nodes. The editor queries for `function_item`, `function_definition`, `method_definition`, `arrow_function`, and similar node types depending on language grammar.

### Fallback (no tree-sitter)

When tree-sitter is unavailable, function detection falls back to:

1. Language-specific patterns (e.g., `fn` keyword for Rust, `def` for Python)
2. Indentation-based body detection

## Language Examples

| Language | `af` selects | `if` selects |
|---|---|---|
| Rust | `fn name(...) { ... }` | Body inside `{ ... }` |
| Python | `def name(...):` + indented body | Indented body only |
| JavaScript | `function name(...) { ... }` or `() => { ... }` | Body inside `{ ... }` |
| Go | `func name(...) { ... }` | Body inside `{ ... }` |

## Closures and Lambdas

`af` and `if` also work on closures/lambdas when tree-sitter identifies them as function-like nodes.

## Nested Functions

With nested functions, the text object selects the innermost function containing the cursor. Move the cursor outside the inner function to select the outer one.

## Operator Examples

| Command | Effect |
|---|---|
| `daf` | Delete entire function |
| `yif` | Yank function body |
| `vaf` | Visually select entire function |
| `cif` | Change function body (enter insert mode) |

## Related

- Class text objects: [/docs/spec/editing/text-objects/class-text-objects.md](/docs/spec/editing/text-objects/class-text-objects.md)
- Text objects overview: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
