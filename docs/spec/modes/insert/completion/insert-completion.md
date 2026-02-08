# Completion in Insert Mode

Autocompletion popup and navigation while typing.

## Triggering Completion

| Key | Source |
|---|---|
| `<C-n>` | Generic: next match (keyword from all sources) |
| `<C-p>` | Generic: previous match |
| `<C-x><C-n>` | Buffer keywords |
| `<C-x><C-l>` | Whole line completion |
| `<C-x><C-f>` | File path completion |
| `<C-x><C-k>` | Dictionary completion |
| `<C-x><C-]>` | Tag completion |
| `<C-x><C-i>` | Include file keywords |
| `<C-x><C-o>` | Omni / LSP completion |
| `<C-x><C-d>` | Definition/macro completion |
| `<C-x><C-u>` | User-defined completion |
| `<C-x>s` | Spell suggestions |

Auto-trigger: completion popup appears automatically after `completion.debounce` ms if `completion.auto` is `true`.

## Popup Navigation

| Key | Action |
|---|---|
| `<C-n>` | Select next item |
| `<C-p>` | Select previous item |
| `<C-y>` | Accept selected item |
| `<C-e>` | Cancel completion, close popup |
| `<CR>` | Accept selected item (if menu visible) |
| `<Tab>` | Accept or cycle (configurable) |
| `<PageDown>` | Scroll popup down |
| `<PageUp>` | Scroll popup up |

## Popup Appearance

| Setting | Default | Description |
|---|---|---|
| `completion.menu.max_items` | `10` | Maximum visible items |
| `completion.menu.border` | `true` | Draw border around popup |
| `completion.menu.show_kind` | `true` | Show completion kind icon |
| `completion.menu.show_source` | `false` | Show source label |

## Info/Documentation Window

When a completion item has documentation, a secondary window appears beside the popup showing the documentation text or type signature.

## Fuzzy Matching

When enabled (`completion.fuzzy = true`), candidates are filtered by fuzzy subsequence match rather than prefix match. Candidates are ranked by match quality.

## Snippet Expansion

LSP snippet completions insert a template with placeholders. After accepting, `<Tab>` jumps to the next placeholder, `<S-Tab>` to the previous.

## Related

- Completion sources: [/docs/spec/modes/insert/completion/insert-completion-sources.md](/docs/spec/modes/insert/completion/insert-completion-sources.md)
- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
