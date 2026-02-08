# Case Changing

Text case transformation operators.

## Case Operators (normative)

| Key | Action | Type |
|---|---|---|
| `~` | Toggle case of character under cursor, advance cursor | Characterwise |
| `g~{motion}` | Toggle case over motion | Operator |
| `g~~` | Toggle case of current line | Linewise |
| `gU{motion}` | Uppercase over motion | Operator |
| `gUU` | Uppercase current line | Linewise |
| `gu{motion}` | Lowercase over motion | Operator |
| `guu` | Lowercase current line | Linewise |

## Visual Mode Case

| Key | Action |
|---|---|
| `~` | Toggle case of selection |
| `U` | Uppercase selection |
| `u` | Lowercase selection |

## Tilde Behavior

The `~` key toggles case of the character under the cursor and moves the cursor right. With a count, `5~` toggles the next 5 characters. When `tildeop` is set, `~` acts as an operator requiring a motion (`~w` toggles a word).

## Case Modifiers in Substitution

| Modifier | Effect |
|---|---|
| `\u` | Uppercase next character |
| `\l` | Lowercase next character |
| `\U` | Uppercase all following until `\E` |
| `\L` | Lowercase all following until `\E` |
| `\E` | End case modification |

## Case Style Conversions

Built-in commands for common case style transformations:

| Style | Example | Description |
|---|---|---|
| camelCase | `helloWorld` | First word lowercase, subsequent capitalized |
| PascalCase | `HelloWorld` | All words capitalized |
| snake_case | `hello_world` | All lowercase with underscores |
| SCREAMING_SNAKE | `HELLO_WORLD` | All uppercase with underscores |
| kebab-case | `hello-world` | All lowercase with hyphens |
| Title Case | `Hello World` | Each word capitalized |

Style conversions are available as commands or can be composed via substitute patterns.

## Unicode Case

Case conversion is Unicode-aware. Characters such as `ß` → `SS` (uppercase), `i` → `I` (except in Turkish locale where `i` → `I` with dot above).

## Related

- Text manipulation: [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)
- Substitute: [/docs/spec/editing/text-manipulation/substitute.md](/docs/spec/editing/text-manipulation/substitute.md)
