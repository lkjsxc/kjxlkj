# Motions

Back: [/docs/spec/editing/README.md](/docs/spec/editing/README.md)
Cursor motion specifications.

## Essential Motions

| Key | Motion | Description |
|-----|--------|-------------|
| `h/j/k/l` | Character | Left/down/up/right |
| `w/W` | Word | Next word start |
| `b/B` | Word | Previous word start |
| `e/E` | Word | Word end |
| `0` | Line | Line start |
| `^` | Line | First non-blank |
| `$` | Line | Line end |
| `gg` | File | File start |
| `G` | File | File end |
| `f{c}` | Find | Find char forward |
| `F{c}` | Find | Find char backward |

## Documents

| Document | Content |
|----------|---------|
| [motions.md](motions.md) | Overview |
| [motion-grammar.md](motion-grammar.md) | Grammar |
| [line-motions.md](line-motions.md) | Line motions |
| [character-find.md](character-find.md) | Char find |
| [word-WORD.md](word-WORD.md) | Word motions |
| [scroll-motions.md](scroll-motions.md) | Scrolling |
| [window-motions.md](window-motions.md) | Window |
| [repeat-motions.md](repeat-motions.md) | Repeat |
| [jumps/](jumps/README.md) | Jump + mark motions |
| [search-motions.md](search-motions.md) | Search |
| [sentence-paragraph.md](sentence-paragraph.md) | Sentence/para |

## Related

- Editing: [docs/spec/editing/README.md](/docs/spec/editing/README.md)
- Text objects: [docs/spec/editing/text-objects/README.md](/docs/spec/editing/text-objects/README.md)
