# Text Objects — Detailed Design

Back: [/docs/design/editing/README.md](/docs/design/editing/README.md)

Design rationale for text objects implementation.

## Overview

Text objects define regions of text for operator application in visual and operator-pending modes.

## Inner vs Around

Every text object has two variants:

| Prefix | Meaning | Example |
|---|---|---|
| `i` | Inner — excludes delimiters | `diw` deletes word only |
| `a` | Around — includes delimiters | `daw` deletes word + surrounding space |

## Implementation Strategy

Text objects are resolved by the mode handler during operator-pending mode. The resolver receives the cursor position and returns a `(start, end)` range.

## Categories

Text object types organized by delimiter style.

### Word Objects

`iw` / `aw` — word boundaries follow Unicode word segmentation (UAX #29). CJK characters are each a single word.

### WORD Objects

`iW` / `aW` — delimited only by whitespace.

### Bracket Objects

`i(` / `a(`, `i[` / `a[`, `i{` / `a{`, `i<` / `a<` — find matching brackets using a bracket-matching stack, respecting nesting depth.

### Quote Objects

`i"` / `a"`, `i'` / `a'` — search current line for matching quotes.

### Tag Objects

`it` / `at` — HTML/XML tag pairs matched via tree-sitter when available, regex fallback otherwise.

### Treesitter Objects

`if` / `af` (function), `ic` / `ac` (class) — use tree-sitter AST nodes.

## Count Handling

A count widens the selection outward for nestable objects (brackets, tags). For non-nestable objects (quotes), count is typically 1.

## Related

- Text objects spec: [/docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
- Operators: [/docs/design/editing/operators-detailed.md](/docs/design/editing/operators-detailed.md)
