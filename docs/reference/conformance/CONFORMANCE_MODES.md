# Conformance: Modes

Back: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## Scope

Current verification status for modal behavior and transitions.

## Ledger

| Requirement ID | Spec Link | Status | Evidence | Notes |
|---|---|---|---|---|
| MODE-001 | `/docs/spec/modes/normal.md` | `verified` | `transition.rs` + `dispatch.rs` + 5 tests | Normal mode dispatch via NormalDispatch |
| MODE-002 | `/docs/spec/modes/insert/README.md` | `verified` | `editor.rs` dispatch_in_mode + 3 tests | Insert mode key handling (chars, enter, backspace, delete, tab) |
| MODE-003 | `/docs/spec/modes/command.md` | `verified` | `editor.rs` dispatch_in_mode + cmdline.rs | Command-line mode with editing, history, cursor movement |
| MODE-004 | `/docs/spec/modes/replace/README.md` | `partial` | `editor.rs` dispatch_in_mode | Single-char replace works; overstrike mode not implemented |
| MODE-005 | `/docs/spec/modes/visual.md` | `scaffold-only` | `transition.rs` | Transition exists; no visual selection tracking |
| MODE-006 | `/docs/spec/modes/transitions.md` | `verified` | `transition.rs` + 5 tests | Normal↔Insert, Normal→Command, Normal→OperatorPending, Normal→Visual |
