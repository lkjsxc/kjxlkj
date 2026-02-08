# Command-Line Mode Mappings

Back: [docs/spec/scripting/mappings/README.md](docs/spec/scripting/mappings/README.md)

Mappings active during command-line (ex) mode input.

## Commands

### Recursive Mapping

`:cmap {lhs} {rhs}` creates a command-line mapping
where the RHS is re-interpreted through mappings.

### Non-Recursive Mapping

`:cnoremap {lhs} {rhs}` creates a command-line
mapping where the RHS is NOT re-interpreted.
Recommended default for user-defined mappings.

### Remove Mapping

`:cunmap {lhs}` removes a command-line mapping.

### Clear All

`:cmapclear` removes all command-line mappings.

## Mapping Semantics

| Property | Behavior |
|----------|----------|
| Trigger | LHS matches typed keys in command-line |
| Expansion | RHS replayed as if typed |
| Timeout | Ambiguous prefix waits `timeoutlen` ms |
| Non-recursive | `cnoremap` skips re-interpretation |
| Priority | Buffer-local has no effect (global only) |

## Common Patterns

### Navigation Shortcuts

Map Ctrl+A to beginning-of-line and Ctrl+E to
end-of-line for readline-style editing.

### History Navigation

Map Up/Down arrows to filtered history search
(matching current input prefix).

### Expansion Shortcuts

Map abbreviations for common command prefixes:
for example, map `%%` to the current file's
directory path.

## Interaction with Completion

When command-line completion is active (tab
completion popup), certain keys have built-in
behavior. User mappings for Tab, Ctrl-n, Ctrl-p
are overridden by the completion system.

## Expression Mappings

`:cnoremap <expr> {lhs} {expr}` evaluates the
expression to produce the RHS string. Useful
for context-sensitive expansions.

## Special Key Handling

### Keys with Built-in Behavior

| Key | Built-in | Can Override |
|-----|----------|--------------|
| `<CR>` | Execute command | Yes |
| `<Esc>` | Cancel command line | Yes |
| `<Tab>` | Trigger completion | Yes |
| `<C-c>` | Cancel (hard) | No |
| `<C-r>` | Insert register | Yes |
| `<C-w>` | Delete word backward | Yes |
| `<C-u>` | Delete to start | Yes |

### Special Key Notation

Same notation as other modes: `<CR>`, `<Esc>`,
`<Tab>`, `<BS>`, `<C-x>`, `<A-x>`, `<Space>`.

## Listing Mappings

`:cmap` with no arguments lists all command-line
mappings. `:verbose cmap` shows where each mapping
was defined.

## Related

- Command-line mode: [docs/spec/modes/cmdline/README.md](docs/spec/modes/cmdline/README.md)
- Mapping modes: [docs/spec/scripting/mappings/mapping-modes.md](docs/spec/scripting/mappings/mapping-modes.md)
