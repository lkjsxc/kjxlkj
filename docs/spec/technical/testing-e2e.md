# Integration, E2E, and Boundary Test Matrix

Back: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

Live PTY matrix for blocker closure.

## Normative E2E Definition

An E2E test must:

1. launch real `kjxlkj` binary in PTY
2. send real terminal bytes/keys
3. capture per-key state dump
4. assert full frame behavior (not trace-only)

## Mandatory Per-Key Dump Fields

- raw input bytes
- normalized key
- resolved action
- mode before and after
- focused pane ID and type
- layout summary with pane rectangles
- frame snapshot or deterministic excerpt
- cursor/caret position

## Mandatory Live Regression Suite (`*R`)

| ID | Risk Addressed | Setup | Action | Deterministic Assertions |
|---|---|---|---|---|
| `WR-01R` | `Shift+a` decode/route drift | Normal mode at EOL | send raw shifted bytes | normalized key is `A`; frame equals physical `A` run |
| `KEYMODE-04R` | `a` incorrectly behaving like `i` at EOL | same line and cursor start | replay `iX<Esc>` and `aX<Esc>` | traces show different pre-insert cursor move and expected final text |
| `CMD-02R` | window-local command scope regression | two-window split on different buffers | execute `:e`, `:w`, `:q` in focused pane | non-focused pane binding and lifecycle remain unchanged |
| `WIN-01R` | split-create-close lifecycle corruption | nested split tree | create/close/only sequence | pane geometry and focus timeline match oracle |
| `WIN-02R` | directional focus ambiguity | asymmetric tree | replay `Ctrl-w h/j/k/l` | focus transitions follow geometry oracle |
| `WIN-03R` | mixed-pane focus bugs | buffer + explorer + terminal | mixed navigation commands | pane type and ID transitions are valid and deterministic |
| `WRAP-11R` | long-line overflow | huge ASCII line | render + scroll | no off-screen writes |
| `WRAP-12R` | wide-grapheme split artifact | huge CJK line | render + resize | no split-wide artifacts |
| `WRAP-14R` | resize storm instability | mixed narrow/wide geometries | repeated resize | bounds invariants always hold |
| `UI-02R` | line-number continuation drift | wrapped long lines | resize + wrap toggle churn | gutter line identity remains deterministic per row |
| `FS-03R` | file IO unreliability | temp file fixture | `:e` edit `:w` reopen | byte-exact round trip and correct modified flag lifecycle |
| `EXP-06R` | explorer stale state under FS drift | explorer visible | external create/rename/delete | tree refresh matches filesystem without corruption |
| `TERM-04R` | PTY resize mismatch | active terminal pane | repeated resize | PTY and pane geometry stay synchronized |
| `TERM-06R` | terminal flood deadlock | flood process + adjacent editor pane | interleave typing and flood | bounded input latency, deterministic focus |
| `JP-06R` | IME leader leakage | active composition | send leader sequence | no explorer action during composition |
| `JP-07R` | IME terminal leakage | active composition | send terminal leader sequence | no terminal action during composition |
| `JP-08R` | composition cancel semantics | active composition | cancel then `Esc` | cancel happens before mode exit |
| `JP-09R` | Japanese rendering under churn | composition + resize + navigation | replay churn script | no half-cell artifacts, cursor remains visible |
| `JP-10R` | command chain during composition | active composition | attempt window-local command chain | command deferred/rejected until IME returns `Idle` |

## Creative Boundary and Race Suite

| ID | Scenario | Acceptance Criterion |
|---|---|---|
| `BD-RACE-01` | terminal flood + explorer refresh + split resize | no panic, bounded latency, deterministic focus |
| `BD-RACE-02` | wrap on/off churn during rapid CJK cursor motion | no half-cell cursor states |
| `BD-RACE-03` | 100-cycle explorer/terminal open-close | no stale IDs, no leaked resources |
| `BD-RACE-04` | `:Explorer` + `:terminal` interleaved under IME | route results remain deterministic |
| `BD-RACE-05` | command scope churn with `:e|w|q` in split layout | target pane rules remain correct each step |

## Failure Artifact Requirements

Every failing `*R` case must include:

- mode before/after
- focused pane ID/type
- layout summary and pane rectangles
- cursor/caret coordinates
- frame diff summary
- last 20 raw inputs and resolved actions

## Release Gate Addendum

Release is green only when:

1. retained baseline suites remain green
2. all mandatory `*R` rows pass with frame/state assertions
3. no high-severity limitation remains open in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Related

- testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- unit baseline: [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
- PTY harness: [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md)
