# Mode Transitions

Mode transitions are deterministic and never block on services.

## Core Rules

| Rule | Description |
|------|-------------|
| Escape returns to Normal | `Esc` from any mode returns to Normal |
| Double escape cancels | `Esc Esc` resets all pending state |
| Entry from Normal only | Mode entry keys work only in Normal mode |

## Transition Table

| From | To | Trigger |
|------|-----|---------|
| Normal | Insert | `i`, `a`, `o`, `O`, `A`, `I` |
| Normal | Visual | `v`, `V`, `Ctrl-v` |
| Normal | Command | `:` |
| Normal | Replace | `R` |
| Insert | Normal | `Esc` |
| Visual | Normal | `Esc` or operator execution |
| Command | Normal | `Esc` or command execution |
| Replace | Normal | `Esc` |

## Invariants

1. No transition MAY block on external IO
2. Transitions MUST be deterministic based on input key
3. Pending operator state clears on mode change
4. Jumplist updated on certain transitions

## Related

- Mode overview: [README.md](README.md)
- Insert mode: [insert/README.md](insert/README.md)
- Replace mode: [replace/README.md](replace/README.md)
