# Integration, E2E, and Boundary Test Matrix

Back: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

High-leverage live matrix for blocker-first reconstruction.

## Normative E2E Definition

An E2E test means all of the following:

1. launch the real `kjxlkj` binary in a user-like terminal environment
2. send real key inputs through PTY bytes or terminal key sequences
3. dump state after each key input
4. assert the visible screen output matches expected rows and pane layout

A trace-only assertion is not sufficient for blocker closure.

## Harness Levels

| Harness | Description | Required For |
|---|---|---|
| Headless state harness | drives core actions without PTY | baseline integration (`T1`) |
| PTY process harness | full binary in PTY with per-key state dumps and frame checks | blocker closure and release (`T2`) |

All `*R` rows below require PTY process harness.

## Per-Key Dump Contract

Each input step must capture:

- raw input bytes
- normalized key
- resolved action
- mode before/after
- focused pane ID/type
- layout summary with pane rectangles
- frame excerpt or full frame snapshot
- cursor/caret position

## Mandatory Live Regression Suite (`*R`)

| ID | Risk Addressed | Setup | Action | Deterministic Assertions |
|---|---|---|---|---|
| `WR-01R` | `Shift+a` decode drift | Normal mode at EOL | send raw shifted bytes | normalization is `A` and final frame matches append-at-EOL oracle |
| `KEYMODE-01` | `Shift+a` route mismatch | same initial state as physical `A` run | replay both runs | per-key dumps and frame timeline are identical |
| `WIN-01R` | split lifecycle corruption | nested split tree | create/close/only operations | pane geometry and focus timeline match oracle |
| `WIN-02R` | directional focus ambiguity | asymmetric tree | replay `Ctrl-w h/j/k/l` | focus transitions match geometry oracle |
| `WIN-03R` | mixed-window focus bugs | buffer + explorer + terminal | directional navigation | pane type/ID transitions are correct |
| `WIN-04R` | resize instability | 3+ pane tree | resize/equalize storm | no invalid pane geometry or focus loss |
| `WIN-05R` | session restore drift | save complex layout | reload session | restored pane tree and focus equal pre-save dump |
| `EXP-01R` | explorer launch unreachable | normal editing session | run `:Explorer` | explorer pane appears and is focused |
| `EXP-02R` | leader mapping route breakage | leader mappings configured | run `<leader>e`, `<leader>E` | toggle/reveal visibility is correct per step |
| `EXP-03R` | open-target routing bugs | explorer on project tree | open via `Enter`, `v`, `s` | target pane and opened path match oracle |
| `EXP-04R` | mixed focus corruption | explorer + buffer + terminal | run mixed `Ctrl-w` routes | focus timeline remains valid |
| `EXP-05R` | long-label wrap corruption | deep paths + badges | scroll/select/wrap | no overflow and stable row identity |
| `EXP-06R` | external FS drift race | explorer visible | mutate filesystem externally | refresh matches filesystem state without corruption |
| `TERM-01R` | terminal launch breakage | normal session | run `:terminal` | terminal pane appears with PTY output |
| `TERM-02R` | launch route inconsistency | leader mappings configured | run `<leader>t`, `<leader>th`, `<leader>tv` | route-to-layout mapping is deterministic |
| `TERM-03R` | mixed navigation drift | mixed-pane session | run `Ctrl-w` commands | navigation semantics match non-terminal panes |
| `TERM-04R` | PTY resize mismatch | active terminal pane | repeated resize | PTY and pane geometry remain synchronized |
| `TERM-05R` | zombie process leak | terminal with active child | close terminal pane | child is reaped within deadline |
| `TERM-06R` | output flood deadlock | terminal flood + neighbor editing | interleave typing and flood | bounded input latency is maintained |
| `TERM-07R` | CJK wrap artifacts | terminal with mixed-script long lines | resize and scroll | no half-cell rendering artifacts |
| `CUR-07R` | cursor disappearance | mode/focus churn | replay churn script | exactly one visible primary cursor |
| `CUR-08R` | width-2 highlight mismatch | wide grapheme line | move cursor onto wide grapheme | both cells are highlighted |
| `CUR-09R` | continuation-cell targeting | narrow geometry + wide text | repeated moves | cursor never targets continuation cell |
| `CUR-10R` | wrap-boundary cursor artifact | wide grapheme at boundary | move across boundary | no split-cursor rendering |
| `CUR-11R` | multi-window cursor ambiguity | rapid focus switching | switch focus repeatedly | one primary cursor only |
| `WRAP-11R` | long-line overflow | 10k ASCII line | render and scroll | no off-screen writes |
| `WRAP-12R` | wide-grapheme split | 10k CJK line | render and resize | no split-wide artifacts |
| `WRAP-13R` | unstable rewrap | long mixed-script line | wrap->nowrap->wrap | breakpoints are deterministic |
| `WRAP-14R` | resize storm overflow | narrow/wide churn | repeated resize | on-screen guarantee holds |
| `WRAP-15R` | tiny geometry panic | repeated 1x1 sizes | render updates | no panic and deterministic clamping |
| `WRAP-16R` | cross-window bound safety | editor + explorer + terminal long content | render all panes | all panes respect bounds |
| `PERF-01R` | missing observability | profiling enabled | burst input then exit | profile line includes required counters |
| `PERF-02R` | hidden O(file) snapshot work | large buffer + small viewport | one render cycle | materialized lines remain bounded |
| `PERF-03R` | idle redraw loop | profiling enabled | idle then quit | no continuous redraw loop reported |
| `JP-06R` | IME leader leakage | active composition | send leader sequence | no explorer action |
| `JP-07R` | IME terminal leakage | active composition | send terminal leader sequence | no terminal action |
| `JP-08R` | composition cancel semantics | active composition | cancel + `Esc` | cancel before mode exit |
| `JP-09R` | IME churn | composition + resize + navigation | replay churn | composition state remains coherent |

## Creative Boundary and Race Suite

| ID | Scenario | Acceptance Criterion |
|---|---|---|
| `BD-RACE-01` | terminal flood + explorer refresh + split resize | no panic, bounded latency, deterministic focus |
| `BD-RACE-02` | wrap on/off churn during rapid CJK cursor moves | no half-cell cursor states or overflow |
| `BD-RACE-03` | 100-cycle explorer/terminal open-close loop | no stale IDs and no leak symptoms |
| `BD-RACE-04` | `:Explorer` and `:terminal` interleaved under IME activity | routing remains deterministic |
| `BD-RACE-05` | rapid split churn while explorer selection updates | pane map and selection identity stay consistent |

## Mandatory Failure Artifacts

Every failing `*R` case must emit:

- mode before/after
- focused pane ID/type
- layout summary and pane rectangles
- cursor/caret coordinates
- top frame excerpt and failing frame diff
- last 20 raw input and resolved action events

## Release Gate Addendum

Release gate is green only when:

1. retained baseline tests remain green
2. all mandatory `*R` rows pass with per-key screen-state assertions
3. no high-severity row remains open in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- PTY harness: [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md)
