# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports only what is currently verified with strong evidence.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is known |
| `partial` | behavior/spec exists but verification is incomplete |
| `blocked` | known user-visible failure or contradiction remains open |
| `unverified` | no trustworthy runtime evidence currently exists |
| `spec-only` | behavior is defined in spec only; implementation not trusted |

## Current Snapshot (2026-02-12)

Current high-confidence statement:

- Documentation baseline is coherent enough to drive reconstruction.
- Implementation artifacts are intentionally removed pending reconstruction.
- Runtime conformance is not accepted as verified at this time.
- User-reported defects are treated as open until closed by new evidence.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Workspace topology contract | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | `verified` | documentation constraints are explicit and internally consistent |
| Command scope semantics (`:q`, `:e`, `:w`) | [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md) | `blocked` | user reports global behavior where window-local behavior is required |
| Insert append semantics (`a` vs `i` vs `A`) | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `blocked` | user reports `a` at EOL behaves like `i` |
| Long-line wrapping and bounds safety | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | `blocked` | user reports overflow and non-wrap behavior |
| File read/write behavior | [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md) | `blocked` | user reports read/write not working reliably |
| Japanese rendering and IME flow | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) | `blocked` | user reports broken Japanese rendering/behavior |
| Test architecture (`T0/T1/T2`) | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `partial` | test strategy documented; authoritative re-validation not yet executed |
| Release readiness | [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) | `blocked` | high-severity limitations open |

## Conformance Closure Rule

No blocker may be moved from `blocked` to `partial` or `verified` without:

1. passing deterministic regression tests for the exact behavior
2. passing corresponding PTY `*R` tests with screen-state assertions
3. synchronized updates to reference and TODO ledgers

## Related

- open blockers: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- mismatch matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- E2E requirements: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
