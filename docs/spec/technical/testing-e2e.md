# Integration, E2E, and Boundary Test Matrix

Back: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

High-leverage live matrix for blocker-first reconstruction.

## Harness Levels

| Harness | Description | Required For |
|---|---|---|
| Headless state harness | drives core actions without PTY | baseline integration (`T1`) |
| PTY process harness | full binary in PTY, raw key bytes, frame capture | blocker closure and release (`T2`) |

All `*R` tests below require PTY harness.

## Mandatory Live Regression Suite (`*R`)

| ID | Risk Addressed | Setup | Action | Deterministic Assertions |
|---|---|---|---|---|
| `WR-01R` | `Shift+a` decode drift | Normal mode at EOL | send raw shifted bytes | normalized key trace is `A`; append semantics match `A` |
| `WIN-01R` | split lifecycle corruption | nested split tree | create/close/only operations | one valid focus; no orphan IDs |
| `WIN-02R` | directional focus ambiguity | asymmetric geometry tree | replay `Ctrl-w h/j/k/l` | focus trace matches geometry oracle |
| `WIN-03R` | mixed-window focus bugs | buffer + explorer + terminal leaves | directional navigation | correct type/ID transitions |
| `WIN-04R` | resize instability | 3+ split tree with long lines | resize/equalize storm | geometry invariants hold |
| `WIN-05R` | session restore drift | save complex layout snapshot | reload session | focused leaf and tree restored |
| `WINNAV-01R` | cyclic navigation drift | mixed-window layout | replay `w/W/p/t/b` script | deterministic golden focus order |
| `WINNAV-02R` | directional + cyclic mismatch | same as above | interleave directional and cyclic commands | no model divergence |
| `WINNAV-03R` | stale previous-focus pointer | close/reopen churn | invoke `Ctrl-w p` after churn | previous target valid or deterministic fallback |
| `WINNAV-04R` | boundary target ambiguity | nested orientation tree | `Ctrl-w t` and `Ctrl-w b` | deterministic top-left and bottom-right targets |
| `WINNAV-05R` | terminal-mode navigation regressions | focused terminal leaf | navigate before/after `Ctrl-\\ Ctrl-n` | identical navigation semantics |
| `WINNAV-06R` | nondeterministic replay | fixed initial state | run same long script twice | traces are byte-identical |
| `EXP-01R` | explorer launch unreachable | normal editing session | run `:Explorer` | explorer leaf appears and is focused |
| `EXP-02R` | leader mapping route breakage | configured leader mappings | run `<leader>e`, `<leader>E` | toggle/reveal paths visible |
| `EXP-03R` | open target routing bugs | explorer on project tree | open via `Enter`, `v`, `s` | correct target window and file path |
| `EXP-04R` | mixed focus corruption | explorer + buffer + terminal | run mixed `Ctrl-w` routes | focus transitions remain valid |
| `EXP-05R` | long-label wrap corruption | deep paths + badges | scroll/select/wrap | no overflow; stable node identity |
| `EXP-06R` | external FS drift race | explorer visible | create/rename/delete outside editor | refresh reflects changes without corruption |
| `TERM-01R` | terminal launch path breakage | normal editing session | run `:terminal` | PTY output appears in leaf |
| `TERM-02R` | launch route inconsistency | leader mappings configured | run `<leader>t`, `th`, `tv` | equivalent spawn semantics |
| `TERM-03R` | terminal leaf navigation drift | mixed-window session | run `Ctrl-w` commands | identical behavior across window types |
| `TERM-04R` | PTY resize mismatch | active terminal split | repeated resize | PTY resize observed; cursor visible |
| `TERM-05R` | zombie process leak | terminal with active output | close terminal leaf | child is reaped within deadline |
| `TERM-06R` | output flood deadlock | terminal flood + editing neighbor | interleave typing and flood | bounded input latency maintained |
| `TERM-07R` | CJK wrap split artifacts | terminal outputs mixed-script long lines | resize and scroll | no half-cell continuation artifacts |
| `CUR-07R` | cursor disappearance | mode/focus churn scenario | replay churn script | primary cursor remains visible |
| `CUR-08R` | width-2 highlight mismatch | line with wide grapheme | move cursor onto wide grapheme | both cells highlighted |
| `CUR-09R` | continuation-cell targeting | narrow geometry + wide text | repeated cursor moves | cursor never targets continuation cell |
| `CUR-10R` | wrap-boundary cursor artifact | wide grapheme at boundary | move across boundary | no split cursor state |
| `CUR-11R` | multi-window cursor ambiguity | rapid focus switching | switch focus repeatedly | exactly one primary cursor visible |
| `WRAP-11R` | long-line overflow | 10k ASCII line | render and scroll | no off-screen writes |
| `WRAP-12R` | wide-grapheme split | 10k CJK line | render and resize | no split-wide artifacts |
| `WRAP-13R` | unstable rewrap | long mixed-script line | wrap->nowrap->wrap | breakpoints deterministic |
| `WRAP-14R` | resize storm overflow | narrow/wide resize churn | repeated resize | on-screen guarantee preserved |
| `WRAP-15R` | tiny geometry panic | repeated 1x1/narrow sizes | render updates | no panic; deterministic clamping |
| `WRAP-16R` | cross-window bound safety | editor + explorer + terminal long content | render all panes | all panes respect bounds |
| `PERF-01R` | missing per-burst observability | profiling enabled | send burst input then exit | `PROFILE` line includes snapshot/render duration and required counters |
| `PERF-02R` | hidden O(file) snapshot behavior | profiling enabled + large multiline buffer + small viewport | render one cycle then exit | `snapshot_materialized_lines_max` is bounded by viewport height plus small margin |
| `PERF-03R` | idle redraw busy-loop regression | profiling enabled | remain idle, then quit | idle probe reports no continuous redraw loop |
| `JP-06R` | IME leader leakage | active composition | send leader sequence | no explorer action during composition |
| `JP-07R` | IME terminal leakage | active composition | send terminal leader sequence | no terminal action during composition |
| `JP-08R` | composition cancel semantics | active composition | cancel + `Esc` | composition cancels before mode exit |
| `JP-09R` | IME state churn | composition + resize + navigation | replay churn | composition state preserved |

## Creative Boundary and Race Suite

| ID | Scenario | Acceptance Criterion |
|---|---|---|
| `BD-RACE-01` | terminal flood + explorer refresh + split resize | no panic; bounded latency; deterministic focus |
| `BD-RACE-02` | wrap on/off churn during rapid CJK cursor moves | no half-cell cursor states and no overflow |
| `BD-RACE-03` | 100-cycle explorer/terminal open-close loop | no stale IDs, no leak symptoms |
| `BD-RACE-04` | `:Explorer` and `:terminal` interleaved under IME activity | routing remains deterministic |

## Mandatory Failure Diagnostics

Every failing live case must report:

- current mode
- focused window ID and type
- layout tree summary
- cursor/caret coordinates
- top frame excerpt
- last 20 raw input and resolved action events

## Release Gate Addendum

Release gate is green only when:

1. retained baseline tests remain green
2. all required `*R` cases pass
3. no high-severity row remains open in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- PTY harness: [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md)
