# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

Open mismatches between target spec and trusted current behavior.

## Baseline (2026-02-12)

- Treat user-reported runtime defects as authoritative blockers.
- Treat existing implementation evidence as stale unless re-verified.
- Treat all blocker closure as PTY E2E gated.

## Open Critical Blockers

| ID | Requirement Link | Observed Gap | Class | Severity | Required Tests | Mandatory Next Action |
|---|---|---|---|---|---|---|
| `LIM-CMD-SCOPE-01` | [/docs/spec/commands/syntax.md](/docs/spec/commands/syntax.md) | `:q`, `:e`, `:w` apply globally instead of focused-window scope | `M1 correctness` | high | `CMD-01`, `CMD-02R`, `WIN-03R` | enforce command execution context as focused window unless command is explicit-global (`:qa`, `:wa`, `:wqa`) |
| `LIM-APPEND-01` | [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md) | `a` at end-of-line behaves like `i` | `M1 correctness` | high | `KEYMODE-02`, `KEYMODE-04R` | fix insert-entry cursor transition for `a` and re-validate against `i` and `A` |
| `LIM-WRAP-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | long lines overflow window instead of deterministic wrap/scroll behavior | `M1 correctness` | high | `WRAP-11R`, `WRAP-12R`, `WRAP-14R` | implement strict wrap/nowrap pipeline with bounds assertions |
| `LIM-FS-IO-01` | [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md) | file read/write behavior is unreliable | `M1 correctness` | high | `FS-01`, `FS-02`, `FS-03R` | restore deterministic FS service integration and command responses |
| `LIM-JP-01` | [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md) | Japanese rendering/composition behavior is incorrect | `M1 correctness` | high | `JP-01`, `JP-04`, `JP-09R` | enforce IME composition boundaries and wide-grapheme rendering safety |
| `LIM-LINENUM-01` | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | per-line number display is not guaranteed for each visible buffer line | `M2 missing feature` | medium | `UI-01`, `UI-02R` | implement gutter numbering contract with deterministic wrap continuation display |

## Open Secondary Gaps

| ID | Requirement Link | Gap | Class | Severity | Next Action |
|---|---|---|---|---|---|
| `LIM-E2E-HARNESS-01` | [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md) | PTY harness evidence quality is insufficient for blocker closure | `M4 verification gap` | medium | complete per-key dump + frame oracle outputs for all `*R` rows |
| `LIM-TODO-COVERAGE-01` | [/docs/todo/README.md](/docs/todo/README.md) | TODO/document linkage must stay exhaustive and directly navigable | `M5 stale docs` | medium | maintain full docs checklist with direct links and checkboxes |

## Deferred Items

| ID | Link | Rationale | Next Review |
|---|---|---|---|
| none | n/a | no non-critical deferrals accepted while high-severity blockers are open | after all high-severity blockers close |

## Closure Rules

A limitation may be closed only when all are true:

1. behavior is reachable from real key/command paths
2. deterministic regression tests pass
3. matching PTY `*R` tests pass with screen-state assertions
4. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md), [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md), and [/docs/todo/README.md](/docs/todo/README.md) are updated together

## Related

- conformance snapshot: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
