# Advanced Editing
Composite workflows built from deterministic primitives.

## Dot repeat

- `.` repeats the last change as a replayable intent.
- Repeat must be stable even if async services update the UI.

## Macros

- Record to register: `q{reg}`
- Stop: `q`
- Replay: `@{reg}` and `@@`

Macro recording/replay uses core intents (not raw terminal bytes).

## Multiple cursors

Canonical spec specification: [docs/spec/features/editing/multicursor.md](docs/spec/features/editing/multicursor.md)
