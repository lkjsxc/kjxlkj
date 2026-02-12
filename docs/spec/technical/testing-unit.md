# Unit and Integration Baseline

Back: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

High-leverage `T0`/`T1` baseline that must exist before blocker `T2` closure.

## Selection Policy

The baseline prefers tests that are:

- directly tied to known blocker behavior
- deterministic with low fixture complexity
- strongly coupled to user-visible outcomes

## Retained Implemented Tests (Preferred)

| Domain | File | Reason to Keep |
|---|---|---|
| mode-entry append | `src/crates/core/kjxlkj-core-state/src/editor.rs` | includes `shift_a_appends_at_eol` regression anchor |
| command-line editing | `src/crates/core/kjxlkj-core-state/src/editor_cmdline_tests.rs` | stable command-entry behavior coverage |
| window commands | `src/crates/core/kjxlkj-core-state/src/editor_wincmd_tests.rs` | deterministic split/focus lifecycle coverage |
| split and explorer flows | `src/crates/core/kjxlkj-core-state/src/editor_stage04_tests.rs` | high-signal mixed-pane flows |
| explorer + terminal state | `src/crates/core/kjxlkj-core-state/src/editor_stage04b_tests.rs` | validates explorer lifecycle and terminal state integration |
| boundary/race | `src/crates/core/kjxlkj-core-state/src/editor_boundary_tests.rs`, `src/crates/core/kjxlkj-core-state/src/editor_race_tests.rs` | catches panics and stale-state drift |
| viewport invariants | `src/crates/core/kjxlkj-core-ui/src/viewport_tests.rs` | wrap/follow/clamp behavior safety |
| key normalization | `src/crates/platform/kjxlkj-input/src/normalize.rs` tests | protects `Shift+a -> A` rule |
| terminal parsing/screen | `src/crates/services/kjxlkj-service-terminal/src/parser_tests.rs`, `src/crates/services/kjxlkj-service-terminal/src/screen_tests.rs` | escape and grid correctness |

## Mandatory Reconstruction Additions

| ID | Tier | Requirement | Acceptance Criterion |
|---|---|---|---|
| `FS-01` | `T1` | focused-window `:e` behavior | non-focused window bindings remain unchanged |
| `FS-02` | `T1` | unsaved guard for `:e` without `!` | command rejected, no retarget |
| `FS-04` | `T1` | write error resilience | modified flag remains true and buffer content unchanged |
| `FS-05` | `T1` | focused `:r` insertion | insertion target is focused buffer only |
| `FS-07` | `T1` | failed read safety | no partial mutations on read failure |
| `CMD-01` | `T1` | `:q` in split layout | only focused window closes |
| `CMD-03` | `T1` | explicit-global command behavior | `:qa`/`:wa` apply globally and only those do |
| `KEYMODE-02` | `T1` | `a` at EOL differs from `i` | exact text and cursor transitions match spec |
| `UI-01` | `T1` | line number per visible row | each rendered buffer row has deterministic line identity |
| `JP-01` | `T1` | Japanese composition commit | committed text is atomic and undo-safe |
| `JP-04` | `T1` | long Japanese wrap behavior | no half-cell artifacts with wide glyphs |

## Minimal Cross-Module Checks

| ID | Scope | Acceptance Criterion |
|---|---|---|
| `CS-01` | snapshot monotonicity | sequence number strictly increases per state update |
| `CS-02` | command-to-intent mapping | parser output maps to expected typed intent |
| `CS-03` | window focus safety | exactly one focused window after each mutation |
| `CS-04` | wrap/nowrap transition safety | no overflow or invalid cursor targets |
| `CS-05` | FS request correlation | request/response IDs map deterministically without cross-talk |

## Coverage Gate Before `T2`

Before running blocker `T2` suites, all of the following must pass:

1. retained preferred tests above
2. mandatory reconstruction additions
3. minimal cross-module checks

## Related

- E2E matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- PTY harness: [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md)
- command scope spec: [/docs/spec/commands/execution-context.md](/docs/spec/commands/execution-context.md)
