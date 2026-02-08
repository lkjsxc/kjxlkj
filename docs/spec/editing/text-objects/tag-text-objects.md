# Tag Text Objects

Back: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)

Text objects for XML/HTML tag pairs.

## Overview

Tag text objects select content within or including HTML/XML tag pairs.

## Text Objects

| Object | Description |
|---|---|
| `it` | Inner tag: content between opening and closing tags |
| `at` | Around tag: including the tags themselves |

## Examples

For `<div>hello</div>` with cursor on `hello`:

| Object | Selects |
|---|---|
| `it` | `hello` |
| `at` | `<div>hello</div>` |

## Nesting

Nested tags are handled correctly:

`<div><span>text</span></div>` — `it` on `text` selects `text`, `at` selects `<span>text</span>`.

## Multi-line

Tag text objects work across multiple lines:

Tag pair spanning lines: `it` selects all content between tags across lines.

## Self-closing Tags

Self-closing tags (`<br/>`, `<img/>`) do not have inner content. `it` on a self-closing tag fails with a bell.

## Attributes

`at` includes the opening tag with all its attributes: `<div class="foo">content</div>` — `at` selects the entire thing.

## Related

- Text objects: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
- Inner text objects: [/docs/spec/editing/text-objects/inner-text-objects.md](/docs/spec/editing/text-objects/inner-text-objects.md)
- Around text objects: [/docs/spec/editing/text-objects/around-text-objects.md](/docs/spec/editing/text-objects/around-text-objects.md)
