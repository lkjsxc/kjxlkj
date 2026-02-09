# TODO: Technical Requirements

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Defining specs

- [/docs/spec/technical/README.md](/docs/spec/technical/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
- [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)

## Testing

- [ ] Per-crate unit tests per testing-unit.md (68 required tests)
- [ ] Integration tests per testing-e2e.md (10 integration scenarios)
- [ ] Headless E2E tests per testing-e2e.md (9 headless scenarios)
- [ ] PTY E2E tests per testing-e2e.md (8 PTY scenarios)
- [ ] Boundary tests per testing-e2e.md (40 boundary scenarios including CJK, session, terminal, wiring)

## Mandatory regression scenarios

- [ ] REG-01: Append at EOL (`a`) never leaves floating cursor
- [ ] REG-02: Long line overflow wraps to next display row
- [ ] REG-03: Leader chords remain reachable (`<leader>e`, `<leader>t`)
- [ ] REG-04: Insert `Enter` persists newline through `:wq`
- [ ] REG-05: tmux/multiplexer smoke edit-save flow
- [ ] REG-06: Japanese/Unicode commit and cancel behavior
- [ ] REG-07: CJK cursor never occupies half-cell position
- [ ] REG-08: Width-2 grapheme at wrap boundary produces padding cell

## Expert-level boundary additions

- [ ] X-PTY-01: Real PTY `:terminal` spawn/output/resize/close lifecycle
- [ ] X-WIN-01: Spatial window neighbor navigation on non-trivial split graph
- [ ] X-EXP-01: Explorer toggle/open/split user flow with window focus checks
- [ ] X-IO-01: Filesystem-backed `:w` and `:e` roundtrip with byte-level assertions
- [ ] X-SES-01: `:SessionSave`/`:SessionLoad` end-to-end layout restore
- [ ] X-IME-01: Japanese composition `Space` candidate cycle without leader leakage
- [ ] X-WRAP-01: Width-2 wrap boundary padding determinism under resize storms

See blueprint: [/docs/log/reconstruction/testing-ideas/2026-02-09-e2e-boundary-blueprint.md](/docs/log/reconstruction/testing-ideas/2026-02-09-e2e-boundary-blueprint.md)

## Technical contracts

- [ ] Contracts per [/docs/spec/technical/contracts.md](/docs/spec/technical/contracts.md)
- [ ] Latency requirements per [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- [ ] Memory requirements per [/docs/spec/technical/memory.md](/docs/spec/technical/memory.md)
- [ ] Profiling per [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)
- [ ] Large files per [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)

## Technical documentation

- [/docs/technical/README.md](/docs/technical/README.md)
- [/docs/technical/unicode.md](/docs/technical/unicode.md)
- [/docs/technical/bidi.md](/docs/technical/bidi.md)
- [/docs/technical/crash-reporting.md](/docs/technical/crash-reporting.md)
- [/docs/technical/debugging.md](/docs/technical/debugging.md)
- [/docs/technical/error-recovery.md](/docs/technical/error-recovery.md)
- [/docs/technical/large-files.md](/docs/technical/large-files.md)
- [/docs/technical/network-fs.md](/docs/technical/network-fs.md)
- [/docs/technical/telemetry.md](/docs/technical/telemetry.md)
- [/docs/technical/testing/README.md](/docs/technical/testing/README.md)
- [/docs/technical/testing/coverage.md](/docs/technical/testing/coverage.md)
- [/docs/technical/testing/load.md](/docs/technical/testing/load.md)
- [/docs/technical/testing/mutation.md](/docs/technical/testing/mutation.md)
- [/docs/technical/testing/regression.md](/docs/technical/testing/regression.md)
