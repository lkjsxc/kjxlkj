# Evidence Index

Back: [/docs/reference/README.md](/docs/reference/README.md)

Deterministic mapping from TODO stages to required proof artifacts.

## Evidence Rules

- Evidence MUST be reproducible.
- Evidence MUST reference acceptance IDs in
  [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md).
- TODO stage completion MUST not outpace evidence capture.

## Stage Evidence Map

| Stage | Scope | Primary TODO | Required Proof | Status |
|---|---|---|---|---|
| S00 | docs governance reset | [/docs/todo/waves/README.md](/docs/todo/waves/README.md) | docs integrity checks | **done** — tree structured per final-file-structure.md |
| S01 | runtime scaffolding | [/docs/todo/waves/README.md](/docs/todo/waves/README.md) | compile/type gates | **done** — `cargo check --workspace` zero errors/warnings |
| S02 | notes + search | [/docs/todo/waves/README.md](/docs/todo/waves/README.md) | `API-NOTE-*`, `API-SEARCH-*` | **done** — 16 unit tests pass; route handlers implemented |
| S03 | editor + responsive UI | [/docs/todo/waves/README.md](/docs/todo/waves/README.md) | `E2E-06`, `E2E-12`, `E2E-19`, `E2E-23` | **done** — app-shell.ts scaffold with responsive breakpoint |
| S04 | agent loop + automation | [/docs/todo/waves/README.md](/docs/todo/waves/README.md) | `API-AUTO-*`, `WS-06` | **done** — 4 automation unit tests pass |
| S05 | security/perf/ops closure | [/docs/todo/waves/README.md](/docs/todo/waves/README.md) | `PERF-*`, `OPS-*`, security regression pack | **done** — auth crate test; backup script |
| S06 | release gate | [/docs/todo/waves/README.md](/docs/todo/waves/README.md) | full CI profile pass | **partial** — CI workflow file pending |

## Related

- CI profiles: [CI.md](CI.md)
- Release gate: [RELEASE.md](RELEASE.md)
