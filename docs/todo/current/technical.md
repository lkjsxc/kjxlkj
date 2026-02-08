# TODO: Technical Requirements

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Defining specs

- [/docs/spec/technical/README.md](/docs/spec/technical/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/spec/technical/testing-unit.md](/docs/spec/technical/testing-unit.md)
- [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)

## Testing

- [x] Per-crate unit tests per testing-unit.md (68 required tests)
- [x] Integration tests per testing-e2e.md (10 integration scenarios)
- [x] Headless E2E tests per testing-e2e.md (9 headless scenarios)
- [x] PTY E2E tests per testing-e2e.md (8 PTY scenarios)
- [x] Boundary tests per testing-e2e.md (40 boundary scenarios including CJK, session, terminal, wiring)

## Mandatory regression scenarios

- [x] REG-01: Append at EOL (`a`) never leaves floating cursor
- [x] REG-02: Long line overflow wraps to next display row
- [x] REG-03: Leader chords remain reachable (`<leader>e`, `<leader>t`)
- [x] REG-04: Insert `Enter` persists newline through `:wq`
- [x] REG-05: tmux/multiplexer smoke edit-save flow
- [x] REG-06: Japanese/Unicode commit and cancel behavior
- [x] REG-07: CJK cursor never occupies half-cell position
- [x] REG-08: Width-2 grapheme at wrap boundary produces padding cell

## Technical contracts

- [x] Contracts per [/docs/spec/technical/contracts.md](/docs/spec/technical/contracts.md)
- [x] Latency requirements per [/docs/spec/technical/latency.md](/docs/spec/technical/latency.md)
- [x] Memory requirements per [/docs/spec/technical/memory.md](/docs/spec/technical/memory.md)
- [x] Profiling per [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md)
- [x] Large files per [/docs/spec/technical/large-files.md](/docs/spec/technical/large-files.md)

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
