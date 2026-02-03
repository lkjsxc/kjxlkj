# Text Objects

Back: [/docs/spec/editing/README.md](/docs/spec/editing/README.md)
Text object specifications for semantic region selection.

## Common Text Objects

| Object | Inner (`i`) | Around (`a`) |
|--------|-------------|--------------|
| Word | `iw` | `aw` |
| WORD | `iW` | `aW` |
| Sentence | `is` | `as` |
| Paragraph | `ip` | `ap` |
| Quotes | `i"` `i'` | `a"` `a'` |
| Brackets | `i(` `i[` `i{` | `a(` `a[` `a{` |
| Tags | `it` | `at` |

## Usage Examples

| Command | Effect |
|---------|--------|
| `diw` | Delete inner word |
| `ci"` | Change inside quotes |
| `dap` | Delete around paragraph |
| `ya{` | Yank around braces |
| `vit` | Select inner tag |

## Documents

| Document | Content |
|----------|---------|
| [text_objects.md](text_objects.md) | Overview |
| [inner-text-objects.md](inner-text-objects.md) | Inner objects |
| [around-text-objects.md](around-text-objects.md) | Around objects |
| [bracket-text-objects.md](bracket-text-objects.md) | Brackets |
| [quote-text-objects.md](quote-text-objects.md) | Quotes |
| [argument-text-objects.md](argument-text-objects.md) | Arguments |
| [function-text-objects.md](function-text-objects.md) | Functions |
| [class-text-objects.md](class-text-objects.md) | Classes |
| [tag-text-objects.md](tag-text-objects.md) | Tags |
| [treesitter-text-objects.md](treesitter-text-objects.md) | Tree-sitter |

## Related

- Editing: [docs/spec/editing/README.md](/docs/spec/editing/README.md)
- Motions: [docs/spec/editing/motions/README.md](/docs/spec/editing/motions/README.md)
