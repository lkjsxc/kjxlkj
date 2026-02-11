# Checklist 01: Critical Blockers

Back: [/docs/todo/checklists/README.md](/docs/todo/checklists/README.md)

## Blocker Scope

- [ ] `LIM-BLOCK-KEY-04` from [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] `LIM-BLOCK-WIN-04` from [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] `LIM-BLOCK-EXP-04` from [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] `LIM-BLOCK-E2E-01` from [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Required Spec Inputs

- [ ] [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- [ ] [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- [ ] [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [ ] [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
- [ ] [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- [ ] [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)

## Required Runtime Closures

- [ ] `Shift+a` and physical `A` are indistinguishable in mode dispatch and screen result
- [ ] split create/close/only operations produce the expected visible pane layout after each input
- [ ] explorer open/toggle/reveal and open-target actions are visibly correct in mixed window layouts
- [ ] blocker E2E suites compare screen snapshots and state dumps, not only action traces

## Required Test IDs

- [ ] `WR-01R`, `KEYMODE-01`, `KEYMODE-02`, `KEYMODE-03`
- [ ] `WIN-01R`, `WIN-02R`, `WIN-03R`, `WIN-04R`, `WIN-05R`
- [ ] `EXP-01R`, `EXP-02R`, `EXP-03R`, `EXP-04R`, `EXP-05R`, `EXP-06R`
- [ ] `BD-RACE-01`, `BD-RACE-03`

## Exit to Next Checklist

- [ ] continue to [02-implementation-architecture.md](02-implementation-architecture.md)
