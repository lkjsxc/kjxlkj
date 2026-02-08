# Insert Mode Mappings

Back: [docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)

Mappings active during Insert mode input processing.

## Commands

### Recursive Mapping

`:imap {lhs} {rhs}` creates an insert-mode mapping
where the RHS is re-interpreted through the mapping
table. Use when chaining mappings is intentional.

### Non-Recursive Mapping

`:inoremap {lhs} {rhs}` creates an insert-mode
mapping where the RHS is NOT re-interpreted.
This is the recommended default for user mappings.

### Remove Mapping

`:iunmap {lhs}` removes an insert-mode mapping.

### Clear All

`:imapclear` removes all insert-mode mappings.

## Mapping Semantics

| Property | Behavior |
|----------|----------|
| Trigger | LHS matches typed keys in Insert mode |
| Expansion | RHS replayed as if typed |
| Timeout | Ambiguous prefix waits `timeoutlen` ms |
| Non-recursive | `inoremap` skips re-interpretation |
| Priority | Buffer-local > global |
| Specificity | Longer prefix > shorter prefix |

## Common Patterns

### Escape Alternatives

Map `jk` or `jj` to `<Esc>` for faster mode exit.
The keybinding resolver MUST wait `timeoutlen` after
the first key before inserting a literal character.

### Snippet Triggers

Map short sequences to expand common code patterns
using `:inoremap` with special key notation for
cursor positioning.

### Auto-Closing Pairs Override

Users may remap `(` to insert `()` and position
cursor between them. This interacts with the
built-in autopairs feature; user mappings override
autopairs for the mapped keys.

## Special Key Interaction

### Ctrl-O

`Ctrl-O` transitions to InsertNormal mode for a
single Normal-mode command. This is a built-in
that MUST NOT be shadowed by default mappings.

### Completion Keys

When the completion popup is visible:

| Key | Default Behavior | Mapping Conflict |
|-----|------------------|------------------|
| `Tab` | Accept completion | Common remap target |
| `Ctrl-n` | Next item | Rare conflict |
| `Ctrl-p` | Previous item | Rare conflict |
| `Ctrl-y` | Confirm selection | Rare conflict |
| `Ctrl-e` | Dismiss popup | Rare conflict |

Completion system takes priority for these keys
unless the user explicitly remaps them.

### Expression Mappings

`:inoremap <expr> {lhs} {expr}` evaluates the
expression and uses the result as the RHS. The
expression can check context (completion visible,
cursor position) to provide conditional mappings.

## Special Key Notation

| Notation | Key |
|----------|-----|
| `<CR>` | Enter / Return |
| `<Esc>` | Escape |
| `<Tab>` | Tab |
| `<BS>` | Backspace |
| `<C-x>` | Ctrl+x |
| `<A-x>` | Alt+x |
| `<Space>` | Space bar |

## Buffer-Local Mappings

`:inoremap <buffer> {lhs} {rhs}` creates a mapping
that only applies to the current buffer. Buffer-local
mappings take priority over global mappings.

## Listing Mappings

`:imap` with no arguments lists all insert-mode
mappings. `:imap {lhs}` shows mappings starting
with the given prefix.

## Related

- Mapping modes: [docs/spec/scripting/mappings/mapping-modes.md](/docs/spec/scripting/mappings/mapping-modes.md)
- Insert mode: [docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- Completion: [docs/spec/modes/insert/completion/insert-completion.md](/docs/spec/modes/insert/completion/insert-completion.md)
