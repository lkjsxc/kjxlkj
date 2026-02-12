# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-level mismatch tracking for reconstruction.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates canonical spec |
| `M2 missing feature` | required capability is absent or unreachable |
| `M3 undocumented behavior` | behavior exists but not canonically specified |
| `M4 verification gap` | behavior may exist but deterministic evidence is insufficient |
| `M5 stale docs` | docs and stronger evidence contradict |

## Matrix

| Req ID | Canonical Document | Requirement | Observed Status | Mismatch Class | Action | Verification Evidence |
|---|---|---|---|---|---|---|
| `R-CMD-SCOPE-01` | [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md) | command execution defaults to focused-window scope | contradiction | `M1` | implement + test-add | user report: `:q/:e/:w` behaving globally |
| `R-APPEND-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `a` at EOL differs from `i`; `A` appends to true EOL | contradiction | `M1` | implement + test-add | user report: `a` behaves like `i` |
| `R-WRAP-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | long lines never overflow viewport bounds | contradiction | `M1` | implement + test-add | user report: non-wrapped overflow |
| `R-FS-01` | [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md) | `:e`, `:r`, `:w` are deterministic and reliable | contradiction | `M1` | implement + test-add | user report: file read/write failure |
| `R-JP-01` | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) | Japanese composition and rendering remain coherent | contradiction | `M1` | implement + test-add | user report: Japanese behavior/rendering is incorrect |
| `R-LINENUM-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | every visible buffer row has deterministic line-number display | spec-only | `M2` | implement + test-add | no trusted runtime proof |
| `R-TEST-E2E-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | blocker closure requires PTY frame assertions | partial | `M4` | test-add | PTY evidence set not yet accepted |
| `R-TEST-HARNESS-01` | [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md) | per-key dumps contain full required artifacts | partial | `M4` | implement + test-add | required diagnostics not consistently proven |
| `R-STRUCT-01` | [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md) | directory/file sizing constraints are explicit | aligned | closed | keep | docs specify <=200-line split + near-12-child targets |
| `R-TODO-LINK-01` | [/docs/todo/README.md](/docs/todo/README.md) | TODO directly links every documentation file | partial | `M5` | spec-update | complete direct-link checklist and keep synchronized |

## Summary

| Class | Open |
|---|---:|
| `M1 correctness` | 5 |
| `M2 missing feature` | 1 |
| `M3 undocumented behavior` | 0 |
| `M4 verification gap` | 2 |
| `M5 stale docs` | 1 |

## Update Rules

- close rows only with reproducible evidence
- whenever a row changes, synchronize `CONFORMANCE`, `LIMITATIONS`, and `TODO`
- when user-reported behavior contradicts old claims, downgrade old claims immediately

## Related

- conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
