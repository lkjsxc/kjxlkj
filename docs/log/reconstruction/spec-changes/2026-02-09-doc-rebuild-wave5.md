# Spec Change Record: 2026-02-09 Doc Rebuild Wave 5

Back: [/docs/log/reconstruction/spec-changes/README.md](/docs/log/reconstruction/spec-changes/README.md)

## Summary

This wave performed documentation-only reconciliation to improve rebuild reliability and remove stale claims.

## Canonical Changes

| Area | Change |
|---|---|
| `docs/README.md` | Rewrote canonical map and reconstruction discipline |
| `docs/spec/README.md` | Clarified authority and high-risk reading order |
| `docs/reference/README.md` | Clarified reference authority model |
| `docs/reference/CONFORMANCE.md` | Replaced overclaims with evidence-based high-level status |
| `docs/reference/LIMITATIONS.md` | Added concrete limitation ledger with evidence and next actions |
| `docs/spec/technical/testing.md` | Replaced stale test mappings with current in-repo suites and required additions |
| `docs/spec/features/terminal/terminal.md` | Strengthened full-scratch terminal and terminal-as-window requirements |
| `docs/spec/features/window/splits-windows.md` | Clarified unified window graph and non-buffer window semantics |
| `docs/spec/features/navigation/file_explorer.md` | Expanded explorer runtime, scaling, and verification requirements |
| `docs/spec/modes/insert/input/insert-japanese-ime.md` | Added explicit IME state machine and interception rules |
| `docs/spec/ux/keybindings/mode-entry.md` | Added shifted printable key normalization requirement |
| `docs/spec/architecture/crates.md` | Added source topology/decomposition constraints |
| `docs/todo/*` | Rebased TODO state to unchecked standby and strict anti-shortcut gates |

## Policy-Aligned Outcomes

- TODO coverage now includes direct links to all documentation files.
- Known high-risk gaps are explicitly carried in `LIMITATIONS`.
- Past wave records are being reduced in favor of canonicalized requirements.

## Evidence

- Audit matrix: [/docs/log/reconstruction/audits/2026-02-09-doc-sync-matrix.md](/docs/log/reconstruction/audits/2026-02-09-doc-sync-matrix.md)
- E2E blueprint: [/docs/log/reconstruction/testing-ideas/2026-02-09-e2e-boundary-blueprint.md](/docs/log/reconstruction/testing-ideas/2026-02-09-e2e-boundary-blueprint.md)
