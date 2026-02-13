# All in Docs Baseline Reset Audit (2026-02-13)

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

Spec/code/todo synchronization audit for All in Docs baseline reset.

## Canonical Sources Used

- [/docs/README.md](/docs/README.md)
- [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md)
- [/docs/policy/README.md](/docs/policy/README.md)
- [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [/docs/todo/README.md](/docs/todo/README.md)

## Consistency Matrix

| Requirement ID | Canonical Document | Requirement Statement | Observed Status | Mismatch Class | Action | Verification Evidence |
|---|---|---|---|---|---|---|
| `AUD-AID-01` | [/docs/overview/all-in-docs.md](/docs/overview/all-in-docs.md) | doctrine must explicitly distinguish All in Docs from docs-only state | aligned | `M5 stale docs` (closed) | `spec-update` | doctrine section now defines the distinction directly |
| `AUD-AID-02` | [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) | typed-language policy must forbid direct JS runtime source | aligned | `M5 stale docs` (closed) | `spec-update` | policy/spec/testing now include `TYPE-01..03` typed gates |
| `AUD-AID-03` | [/docs/todo/README.md](/docs/todo/README.md) | TODO checkboxes must be reset for fresh reconstruction start | aligned | `M5 stale docs` (closed) | `spec-update` | `rg -n "\[x\]|\[X\]" docs/todo` returns no matches |
| `AUD-AID-04` | [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md) | derived runtime artifacts may be removed in baseline reset | aligned | `M2 missing feature` (closed) | `implement` | `NO_SRC_DIR`, `NO_STATIC_DIR`, `NO_CARGO_MANIFESTS` checks pass |
| `AUD-AID-05` | [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md) | docs Docker launch must start and serve canonical docs | aligned | `M2 missing feature` (closed) | `implement` | compose config valid, health becomes `healthy`, `/README.md` returns `200` |
| `AUD-AID-06` | [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) | ledgers must reflect reset baseline and open blockers | aligned | `M5 stale docs` (closed) | `spec-update` | conformance/limitations/drift/release/CI rewritten consistently |

## Closed Mismatches

1. `AUD-AID-01` All in Docs doctrine clarified and made explicit.
2. `AUD-AID-02` typed runtime requirements and no-JS rule made normative and testable.
3. `AUD-AID-03` TODO and waves reset to unchecked state.
4. `AUD-AID-04` derived runtime artifacts removed from repository baseline.
5. `AUD-AID-05` docs compose startup verified with healthy status.
6. `AUD-AID-06` reference ledgers synchronized to reset baseline.

## Deferred/Pending Mismatches

| ID | Reason Deferred | Next Action |
|---|---|---|
| `DEF-RUNTIME-04` | runtime/API/WS/UI evidence intentionally deferred to reconstruction waves | execute Stage 01 onward and close `LIM-RUNTIME-04`, `LIM-API-04`, `LIM-WS-04`, `LIM-UI-04` |
| `DEF-TYPE-01` | typed compile/type gates require rebuilt runtime artifacts | rebuild typed backend/frontend skeleton and pass `TYPE-01..02` |
| `DEF-QUALITY-04` | regression/perf/ops evidence requires reconstructed runtime | rerun `REG-*`, `PERF-*`, and `OPS-*` packs post-reconstruction |

## Deterministic Checks

- `Check:` `test ! -d src && echo NO_SRC_DIR`
- `Result:` pass
- `Proof:` `NO_SRC_DIR`
- `Check:` `test ! -d static && echo NO_STATIC_DIR`
- `Result:` pass
- `Proof:` `NO_STATIC_DIR`
- `Check:` `test ! -f Cargo.toml && test ! -f Cargo.lock && echo NO_CARGO_MANIFESTS`
- `Result:` pass
- `Proof:` `NO_CARGO_MANIFESTS`
- `Check:` `find . -type f -name '*.js' | wc -l`
- `Result:` pass
- `Proof:` `0`
- `Check:` `docker compose config`
- `Result:` pass
- `Proof:` config renders and service `docs` is valid
- `Check:` docs launch smoke (`up`, health, `/README.md`, `down`)
- `Result:` pass
- `Proof:` health `healthy`, HTTP status `200`
