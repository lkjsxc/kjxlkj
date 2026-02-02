# Text Objects
Text objects define a region for an operator to act upon.

## Requirements
- Text object selection is deterministic and based on buffer snapshots.
- Language-aware text objects (e.g. function, class) may exist, but must be async-backed and cached.

## Core text objects (normative)

- Words: `iw` `aw` `iW` `aW`
- Quotes: `i"` `a"` `i'` `a'` ``i` `` ``a` ``
- Brackets/braces: `i(` `a(` `i[` `a[` `i{` `a{` `i<` `a<`
- Paragraph/sentence: `ip` `ap` `is` `as`
- Tags: `it` `at` (HTML/XML)

## Related

- Motions: [docs/spec/editing/motions/motions.md](docs/spec/editing/motions/motions.md)
- Operators: [docs/spec/editing/operators/operators.md](docs/spec/editing/operators/operators.md)
- Syntax engine: [docs/spec/features/syntax/syntax.md](docs/spec/features/syntax/syntax.md)
