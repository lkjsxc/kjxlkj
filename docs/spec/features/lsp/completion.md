# Auto-Completion

Intelligent code completion from multiple sources.

## Triggering Completion (normative)

### Automatic

Completion popup appears automatically after typing an identifier character when `auto_complete = true` (default). A debounce delay of `completion_delay` ms (default 100) prevents excessive requests.

### Manual

| Key | Action |
|---|---|
| `<C-Space>` | Trigger completion menu |
| `<C-n>` | Next item (also triggers if menu closed) |
| `<C-p>` | Previous item (also triggers if menu closed) |

## Completion Sources (normative)

| Source | Priority | Content |
|---|---|---|
| LSP | 1 (highest) | Semantic completions from language server |
| Snippets | 2 | Snippet templates |
| Buffer words | 3 | Words from current and open buffers |
| File paths | 4 | File and directory paths (triggered by `/` or `\`) |

Sources are merged and sorted by priority, then by match score.

## Menu Display

The completion menu shows up to `completion_menu_max` items (default 20). Each entry displays:

| Column | Content |
|---|---|
| Icon | Completion kind (function, variable, struct, etc.) |
| Label | Completion text |
| Source | Source indicator |

### Kind Icons

| Icon | Kind |
|---|---|
| `f` | Function/Method |
| `S` | Struct/Class |
| `E` | Enum |
| `v` | Variable |
| `M` | Module |
| `k` | Keyword |
| `s` | Snippet |
| `p` | Property/Field |
| `C` | Constant |
| `i` | Interface/Trait |

## Navigation

| Key | Action |
|---|---|
| `<C-n>` | Next item |
| `<C-p>` | Previous item |
| `<C-y>` | Accept selected |
| `<C-e>` | Cancel menu |
| `<Tab>` | Accept (when configured) |
| `<S-Tab>` | Previous item (when configured) |

## Documentation Preview

When a completion item is selected, its documentation appears in a side panel. The panel shows the full type signature and doc comment.

## Fuzzy Matching

Completion uses fuzzy matching by default. Typing `fn` matches `function`, `format_number`, `find_next`. Matching is case-insensitive unless the query contains uppercase characters.

## Configuration

| Option | Default | Description |
|---|---|---|
| `auto_complete` | `true` | Enable automatic completion |
| `completion_delay` | `100` | Debounce delay in ms |
| `completion_menu_max` | `20` | Max items in menu |
| `completion_doc_preview` | `true` | Show documentation panel |

## Performance

Completion requests are debounced and cached. Repeated completions at the same position reuse cached results. LSP completion requests include the `triggerKind` field to let the server optimize.

## Related

- Completion sources detail: [/docs/spec/features/editing/insert-completion-sources.md](/docs/spec/features/editing/insert-completion-sources.md)
- Insert completion: [/docs/spec/features/editing/insert-completion.md](/docs/spec/features/editing/insert-completion.md)
- Signature help: [/docs/spec/features/lsp/signature-help.md](/docs/spec/features/lsp/signature-help.md)
