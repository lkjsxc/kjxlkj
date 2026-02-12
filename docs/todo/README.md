# Reconstruction TODO

Back: [/docs/README.md](/docs/README.md)

`/docs/todo/` is the execution contract for rebuilding `kjxlkj` from docs.

## Start Gate

- [ ] read [/README.md](/README.md)
- [ ] read [/docs/README.md](/docs/README.md)
- [ ] read [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [ ] read [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] read [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [ ] read [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] read [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [ ] read [/docs/spec/technical/testing-pty-harness.md](/docs/spec/technical/testing-pty-harness.md)

## Immediate Blocker Pack (Must Close First)

- [ ] `LIM-CMD-SCOPE-01`: command scope correctness
  [/docs/spec/commands/execution-context.md](/docs/spec/commands/execution-context.md)
  + tests `CMD-01`, `CMD-02R`
- [ ] `LIM-APPEND-01`: `a` at EOL correctness
  [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
  + tests `KEYMODE-02`, `KEYMODE-04R`
- [ ] `LIM-WRAP-01`: long-line wrap correctness
  [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
  + tests `WRAP-11R`, `WRAP-12R`, `WRAP-14R`
- [ ] `LIM-FS-IO-01`: file IO reliability
  [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md)
  + tests `FS-03R`, `FS-04`, `FS-07`
- [ ] `LIM-JP-01`: Japanese/IME correctness
  [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)
  + tests `JP-09R`, `JP-10R`
- [ ] `LIM-LINENUM-01`: line number per visible row
  [/docs/spec/features/ui/gutter-line-numbers.md](/docs/spec/features/ui/gutter-line-numbers.md)
  + tests `UI-01`, `UI-02R`

## Critical Documents (Re-read in Every Wave)

- [ ] [/docs/spec/README.md](/docs/spec/README.md)
- [ ] [/docs/spec/commands/execution-context.md](/docs/spec/commands/execution-context.md)
- [ ] [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- [ ] [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [ ] [/docs/spec/features/ui/gutter-line-numbers.md](/docs/spec/features/ui/gutter-line-numbers.md)
- [ ] [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)
- [ ] [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [ ] [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- [ ] [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [ ] [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)

## Wave Program

- [ ] open [/docs/todo/waves/README.md](/docs/todo/waves/README.md)
- [ ] execute stages and waves in order only
- [ ] do not mark any checkbox complete without deterministic evidence
- [ ] keep directories near 12 direct children and split files before 200 lines

## Documentation Coverage Checklist

TODO must directly link every documentation file.

- [ ] open [/docs/todo/doc-index/README.md](/docs/todo/doc-index/README.md)
- [ ] complete [/docs/todo/doc-index/checklist-001.md](/docs/todo/doc-index/checklist-001.md)
- [ ] complete [/docs/todo/doc-index/checklist-002.md](/docs/todo/doc-index/checklist-002.md)
- [ ] complete [/docs/todo/doc-index/checklist-003.md](/docs/todo/doc-index/checklist-003.md)
- [ ] complete [/docs/todo/doc-index/checklist-004.md](/docs/todo/doc-index/checklist-004.md)
- [ ] complete [/docs/todo/doc-index/checklist-005.md](/docs/todo/doc-index/checklist-005.md)

## Completion Gate

- [ ] all stage and wave checklists are complete in order
- [ ] all high-severity limitation rows are closed
- [ ] all mandatory `*R` tests pass with frame assertions
- [ ] reference ledgers and TODO are synchronized in one logical change
- [ ] release gate in [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) is satisfied
