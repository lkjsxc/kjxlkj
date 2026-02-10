# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-by-requirement mismatch tracking.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates spec |
| `M2 missing feature` | required behavior is absent or unreachable |
| `M3 undocumented behavior` | runtime behavior exists without spec coverage |
| `M4 verification gap` | behavior may exist but lacks deterministic tests |
| `M5 stale docs` | docs claim obsolete or unverified behavior |

## Active Matrix

| Req ID | Canonical Spec | Expected Behavior | Observed Gap | Class | Action | Status |
|---|---|---|---|---|---|---|
| R-KEY-01 | `/docs/spec/ux/keybindings/mode-entry.md` | `Shift+a` dispatches as `A` | shifted key path not reliably normalized | M1 | fix input normalization + regression | open |
| R-CUR-01 | `/docs/spec/editing/cursor/README.md` | `a` at EOL inserts after last grapheme | behaves like `i` in failing path | M1 | fix append logic + regression | open |
| R-EXP-01 | `/docs/spec/features/navigation/file_explorer.md` | explorer launch commands open explorer window | launch path not reliably wired | M2 | implement wiring + E2E | open |
| R-TERM-01 | `/docs/spec/features/terminal/terminal.md` | `:terminal` opens PTY-backed window | terminal path remains scaffold-only | M2 | implement PTY path + E2E | open |
| R-WIN-01 | `/docs/spec/features/window/splits-windows.md` | mixed-window navigation correctness | focus transitions inconsistent | M1 | fix traversal + E2E | open |
| R-I18N-01 | `/docs/spec/modes/insert/input/insert-japanese-ime.md` | IME compose/cancel/leader isolation | composition path incorrect | M1 | implement state routing + tests | open |
| R-WRAP-01 | `/docs/spec/features/ui/viewport.md` | no off-screen overflow on long lines | overflow still observed | M1 | enforce wrap invariants + tests | open |
| R-TODO-01 | `/docs/todo/current/README.md` | TODO reflects real open work | historical checklists marked complete | M5 | rebuild TODO phases and gates | closed |
| R-DOCS-01 | `/docs/todo/doc-coverage/README.md` | every doc linked directly in TODO | stale coverage list | M5 | regenerate coverage after cleanup | open |
| R-ARCH-01 | `/docs/spec/architecture/source-layout.md` | around-12 child fan-out and file split discipline | not yet enforced in rebuild flow | M4 | enforce in phase 5 checks | open |

## Update Rules

- close a row only with linked deterministic evidence
- if deferred, keep `status=open` and record next concrete action
- update this file in the same change as conformance and limitations

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Verification gates: [/docs/todo/current/verification.md](/docs/todo/current/verification.md)
