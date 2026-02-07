# Cursor semantics

Back: [/docs/spec/editing/README.md](/docs/spec/editing/README.md)

Cursor behavior is core-owned, deterministic, and defined over buffer snapshots.

## Requirements
- Cursor is a `(line, column)` semantic position with explicit rules for clamping.
- Motions never panic; they clamp or no-op per rule.
- Display width (graphemes) is a renderer concern; cursor semantics operate on text indices with a stable mapping.

## Column model

This project uses a mode-dependent column model.

- **End-exclusive cursor**: the maximum valid column on a non-empty line is the last character index.
	- Valid columns for a line of length $N>0$: $0..N-1$
	- For an empty line ($N=0$): only column $0$ is valid.
- **End-inclusive cursor (insertion point)**: the cursor may also be positioned one past the last character.
	- Valid columns for a line of length $N$: $0..N$

### Which modes use which model

- Normal / Visual / VisualLine / Replace: **end-exclusive**
- Insert: **end-inclusive** (cursor is an insertion point)

## Append semantics (`a`)

In Normal mode, `a` enters Insert mode with the insertion point after the character under the cursor.

Given a line of length $N$ and current cursor column $c$ in Normal mode:

- The insertion column is `min(c + 1, N)`.
- Then mode becomes Insert.

This means:

- If the cursor is on the last character ($c = N-1$), `a` enters Insert with column $N$ (append at end-of-line).
- If the line is empty ($N=0$), `a` enters Insert at column 0.

### Repeated `a` then `Esc` regression guard (normative)

If the user repeatedly presses `a` and then exits Insert with `Esc`, the resulting Normal-mode cursor MUST clamp to the last character of the line (or `0` on empty line).

The cursor MUST NOT remain at a floating end-inclusive column in Normal mode.

| Line length | Expected Normal-mode column after `a ... Esc` |
|---|---|
| `N = 0` | `0` |
| `N > 0` | `N - 1` |

## Mode transition clamping

When transitioning into an end-exclusive mode (Normal/Visual/Replace), the cursor MUST be clamped into the end-exclusive range for the active buffer/line.

This clamping rule is mandatory even after rapid mode churn (`a`, typed characters, `Esc`, repeated many times).

## Related documentation

See [docs/design/editing/README.md](/docs/design/editing/README.md) for design rationale on editing primitives.
