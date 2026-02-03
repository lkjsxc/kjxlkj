# Normal mode
Normal mode is the primary composition mode: motions + operators + commands.

## Requirements
- Normal mode interprets keys into typed intents only.
- All edits are applied by core as transactions.
- Async results (LSP, git, syntax) may influence UI, but not block input.

## Responsibilities

- Navigation (motions)
- Range composition (operator + motion/text object)
- Mode entry (insert/visual/command/replace)
- Repeatability (counts, dot-repeat)

## Insert entry keys

- `i` enters Insert at the current cursor (insert before the character under cursor).
- `a` enters Insert after the character under the cursor (append semantics).
	- Cursor rules are defined in: [docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)

## Related

- Motions/operators/text objects: [docs/spec/editing/README.md](/docs/spec/editing/README.md)
- Commands: [docs/spec/commands/README.md](/docs/spec/commands/README.md)
- Keybinding hints: [docs/spec/features/config/keybinding_hints.md](/docs/spec/features/config/keybinding_hints.md)
