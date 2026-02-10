# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-by-requirement mismatch tracking for standby baseline.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates spec |
| `M2 missing feature` | required behavior not implemented or unreachable |
| `M3 undocumented behavior` | runtime behavior exists without spec coverage |
| `M4 verification gap` | behavior exists but lacks deterministic tests |
| `M5 stale docs` | docs contain obsolete or conflicting claims |

## Standby Baseline Matrix

| Req ID | Canonical Spec | Expected Behavior | Current Gap | Class | Next Action | Status |
|---|---|---|---|---|---|---|
| R-BASELINE-01 | `/docs/spec/README.md` | full implementation conforming to spec | implementation absent by design | M2 | execute TODO phases from standby | open |
| R-KEY-01 | `/docs/spec/ux/keybindings/mode-entry.md` | `Shift+a` dispatches as `A` | not implemented in standby baseline | M2 | implement + `WR-01` | open |
| R-CUR-01 | `/docs/spec/editing/cursor/README.md` | `a` at EOL appends after last grapheme | not implemented in standby baseline | M2 | implement + `WR-02`/`WR-08` | open |
| R-EXP-01 | `/docs/spec/features/navigation/file_explorer.md` | explorer launch and split-open wiring | not implemented in standby baseline | M2 | implement + `HE-04`/`HE-05`/`WR-05` | open |
| R-TERM-01 | `/docs/spec/features/terminal/terminal.md` | PTY-backed terminal window path | not implemented in standby baseline | M2 | implement + `HE-06`/`WR-03`/`PE-01` | open |
| R-WIN-01 | `/docs/spec/features/window/splits-windows.md` | mixed-window `Ctrl-w` correctness | not implemented in standby baseline | M2 | implement + `WR-06`/`PE-05` | open |
| R-I18N-01 | `/docs/spec/modes/insert/input/insert-japanese-ime.md` | IME composition and leader isolation | not implemented in standby baseline | M2 | implement + `JP-01` to `JP-05` | open |
| R-WRAP-01 | `/docs/spec/features/ui/viewport.md` | no off-screen long-line overflow | not implemented in standby baseline | M2 | implement + `WR-07`/`BD-*` | open |
| R-TODO-01 | `/docs/todo/current/README.md` | TODO is standby-ready and evidence-gated | must remain synchronized throughout rebuild | M4 | enforce verification gate on every phase | open |

## Update Rules

- close rows only with deterministic evidence
- keep deferred rows open with explicit next action
- update this file alongside `CONFORMANCE` and `LIMITATIONS`

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Verification gates: [/docs/todo/current/verification.md](/docs/todo/current/verification.md)
