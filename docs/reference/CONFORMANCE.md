# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger records what is currently verified.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | reachable behavior confirmed with deterministic evidence |
| `partial` | behavior exists but has user-visible gaps |
| `scaffold-only` | stubs/types exist but user path is incomplete |
| `unverified` | no current deterministic evidence |

## Current Snapshot (2026-02-10)

This repository is in documentation-first reconstruction mode.

## Domain Summary

| Domain | Status | Notes |
|---|---|---|
| Architecture contracts | `partial` | crate topology defined; runtime wiring needs re-verification |
| Core editing and modes | `partial` | known `a`/`A` and shifted key regressions remain |
| Command dispatch | `partial` | essential paths exist, but launch wiring gaps remain |
| Window/split behavior | `partial` | mixed-window navigation and integration gaps remain |
| Explorer integration | `partial` | launch and split-open wiring not reliably closed |
| Terminal integration | `scaffold-only` | PTY-backed first-class window path not verified end-to-end |
| Unicode/CJK/IME | `partial` | Japanese composition and leader isolation issues open |
| Rendering and wrapping | `partial` | no-overflow guarantee requires stricter verification |
| Session restore | `partial` | behavior exists, but mixed window restoration needs closure |
| LSP/Git/Index services | `scaffold-only` | service surface incomplete |
| Accessibility | `unverified` | no deterministic evidence yet |

## Evidence Rules

A claim is valid only when all are true:

1. linked normative spec exists
2. behavior is reachable from real input path
3. deterministic verification evidence exists
4. any remaining user-visible gap is listed in `LIMITATIONS`

## Related

- Open gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Mismatch matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Target behavior: [/docs/spec/README.md](/docs/spec/README.md)
