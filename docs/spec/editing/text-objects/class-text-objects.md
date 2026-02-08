# Class Text Objects

Text objects for selecting class/struct/enum definitions.

## Commands (normative)

| Object | Description |
|---|---|
| `ic` | Inner class: class body/members only (excluding declaration) |
| `ac` | Around class: entire class (declaration + body + closing) |

## Detection Method

Uses tree-sitter AST nodes: `struct_item`, `class_declaration`, `enum_item`, `interface_declaration`, `impl_item`, and similar node types.

### What Counts as "Class"

| Language | Matched Constructs |
|---|---|
| Rust | `struct`, `enum`, `impl`, `trait` |
| Python | `class` |
| JavaScript/TypeScript | `class`, `interface`, `enum` |
| Go | `type ... struct`, `type ... interface` |
| Java/C# | `class`, `interface`, `enum` |

## Behavior

- `ic` selects only the members/body between delimiters (e.g., inside `{ }`)
- `ac` selects the entire construct including keywords, name, and delimiters

## Nested Classes

The text object selects the innermost class containing the cursor. Move cursor outside the inner class to operate on the outer one.

## Operator Examples

| Command | Effect |
|---|---|
| `dac` | Delete entire class/struct |
| `yic` | Yank class body |
| `vac` | Visually select entire class |
| `cic` | Change class body |

## Related

- Function text objects: [/docs/spec/editing/text-objects/function-text-objects.md](/docs/spec/editing/text-objects/function-text-objects.md)
- Text objects overview: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
