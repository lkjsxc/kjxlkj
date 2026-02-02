# Registers
Registers store text for yank/delete and for macros.

## Requirements
- Register updates are core-owned and transactional.
- Clipboard interaction is isolated behind a platform boundary; core state remains deterministic.
- Macros replay typed intents (not raw terminal bytes) to preserve reproducibility.

## Register set (normative)

- `"` unnamed
- `0-9` numbered history
- `a-z` named (lowercase replace, uppercase append)
- `:`, `.`, `%` read-only (last command, last insert, current file)
- `+`, `*` system clipboard
- `_` black hole

## Related

- Advanced editing (macros): [docs/spec/editing/operators/advanced.md](docs/spec/editing/operators/advanced.md)
