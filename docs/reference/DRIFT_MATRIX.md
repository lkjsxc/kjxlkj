# Drift Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Requirement-by-requirement mismatch tracking for current state.

## Mismatch Classes

| Class | Meaning |
|---|---|
| `M1 correctness` | runtime behavior violates spec |
| `M2 missing feature` | required behavior is not implemented or not reachable |
| `M3 undocumented behavior` | behavior exists without canonical spec |
| `M4 verification gap` | behavior exists but deterministic coverage is insufficient |
| `M5 stale docs` | docs claim status contradicted by stronger evidence |

## Closed Drift Rows

| Req ID | Original Class | Resolution | Evidence |
|---|---|---|---|
| `R-BASELINE-01` | `M2 missing feature` | ✓ 19 crates implemented | cargo check passes, 77 source files |
| `R-KEY-01` | `M2 missing feature` | ✓ shift normalization verified | 8 input decode tests in decode.rs |
| `R-KEY-02` | `M2 missing feature` | ✓ decode precedes mode | decode.rs architecture |
| `R-CUR-01` | `M2 missing feature` | ✓ cursor dispatch verified | mode dispatch tests |
| `R-CUR-02` | `M2 missing feature` | ✓ grapheme-safe cursor | grapheme_width and grid.rs tests |
| `R-WIN-01` | `M2 missing feature` | ✓ unified WindowTree | 7 tests in kjxlkj-core-state |
| `R-WIN-02` | `M2 missing feature` | ✓ split operations | split.rs with close/rebalance |
| `R-WIN-03` | `M2 missing feature` | ✓ Ctrl-w dispatch | normal.rs + focus_next/prev |
| `R-EXP-01` | `M2 missing feature` | ✓ explorer launch paths | ExplorerService crate |
| `R-EXP-02` | `M2 missing feature` | ✓ explorer navigation | 4 explorer state tests |
| `R-TERM-01` | `M2 missing feature` | ✓ terminal service | Screen, Parser, 7 tests |
| `R-WRAP-01` | `M2 missing feature` | ✓ overflow protection | 5 wrap tests in kjxlkj-render |
| `R-WRAP-02` | `M2 missing feature` | ✓ width-2 boundary | wide_at_boundary test |
| `R-ARCH-01` | `M2 missing feature` | ✓ grouped crate paths | src/crates/{app,core,platform,services} |
| `R-ARCH-02` | `M2 missing feature` | ✓ workspace manifest | 19 crates in Cargo.toml |
| `R-LOG-01` | `M5 stale docs` | ✓ retention discipline | wave logs cleaned |
| `R-I18N-01` | `M4 verification gap` | ✓ IME composition isolation | JP-03, JP-04, JP-05, JP-06, JP-07 tests |
| `R-I18N-02` | `M4 verification gap` | ✓ IME stable under churn | JP-08R, JP-09R tests |

## Open Drift Rows

| Req ID | Canonical Spec | Expected Behavior | Current Gap | Class | Next Action | Status |
|---|---|---|---|---|---|---|
| `R-TERM-02` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | bounded latency PTY I/O | unit tests only; no live E2E | `M4 verification gap` | PTY E2E harness | open |
| `R-TEST-01` | [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) | high-risk features proven by E2E | 89 unit tests; no PTY E2E | `M4 verification gap` | PTY E2E harness | open |

## Summary

| Class | Closed | Open |
|---|---:|---:|
| `M1 correctness` | 0 | 0 |
| `M2 missing feature` | 15 | 0 |
| `M3 undocumented behavior` | 0 | 0 |
| `M4 verification gap` | 2 | 2 |
| `M5 stale docs` | 1 | 0 |

## Update Rules

- close a row only with reproducible evidence
- close high-severity rows before release claims
- update this file together with `CONFORMANCE`, `LIMITATIONS`, and TODO state

## Related

- Conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- TODO verification gates: [/docs/todo/current/verification.md](/docs/todo/current/verification.md)
