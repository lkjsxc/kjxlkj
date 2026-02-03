# Visual Mode

Visual modes provide explicit selection regions for operator actions.

## Variants

| Mode | Entry | Selection Type |
|------|-------|---------------|
| Visual | `v` | Character-wise |
| Visual Line | `V` | Line-wise |
| Visual Block | `Ctrl-v` | Block/column |

## Selection Behavior

| Aspect | Behavior |
|--------|----------|
| Anchor | Fixed at entry position |
| Cursor | Extends selection boundary |
| Motions | Move cursor, expand/shrink selection |
| `o` | Swap anchor and cursor ends |

## Operators in Visual Mode

| Key | Action |
|-----|--------|
| `d` | Delete selection |
| `y` | Yank selection |
| `c` | Change selection |
| `>` | Indent selection |
| `<` | Outdent selection |
| `~` | Toggle case |

## Exit

| Trigger | Result |
|---------|--------|
| `Esc` | Cancel, return to Normal |
| Operator | Execute on selection, return to Normal |

## Related

- Editing: [docs/spec/editing/README.md](/docs/spec/editing/README.md)
- Visual details: [docs/spec/editing/visual/README.md](/docs/spec/editing/visual/README.md)
- Multiple cursors: [docs/spec/features/editing/multicursor.md](/docs/spec/features/editing/multicursor.md)
