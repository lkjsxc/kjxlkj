# Insert Mode Mapping Specification

Back: [/docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)

Insert-mode mappings are registered with `imap` (recursive) or `inoremap` (non-recursive).

## Mapping semantics (normative)

| Property | Behavior |
|---|---|
| Trigger | LHS key sequence matches input while in Insert mode |
| Expansion | RHS is replayed as if typed; special key notation is expanded |
| Timeout | Ambiguous prefixes wait `timeoutlen` ms (default 1000) |
| Non-recursive | `inoremap` does NOT re-interpret the RHS through the mapping table |
| Priority | Buffer-local > global; more-specific prefix > less-specific |

## Escape alternatives

A common pattern maps `jk` or `jj` to `<Esc>` for faster mode exit.

The keybinding resolver MUST wait `timeoutlen` after `j` before inserting a literal `j`. If `k` arrives within the timeout, the mapping fires and mode transitions to Normal.

## Ctrl-O for single Normal command

`Ctrl-O` in Insert mode is NOT a mapping; it is a built-in that transitions to `InsertNormal` mode. Mappings MUST NOT shadow `Ctrl-O` by default.

## Interaction with completion

When the completion popup is visible, mappings for `Tab`, `Ctrl-n`, `Ctrl-p` may conflict with completion navigation. The completion system takes priority for these keys unless the user explicitly remaps them.

## Related

- Insert mode spec: [/docs/spec/modes/insert/insert-mappings.md](/docs/spec/modes/insert/insert-mappings.md)
- Mapping modes: [/docs/spec/scripting/mappings/mapping-modes.md](/docs/spec/scripting/mappings/mapping-modes.md)
- Input decoding: [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
