# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Mandatory verification contract for reconstruction.

## Verification Tiers

| Tier | Purpose | Required Evidence |
|---|---|---|
| `T0` | local invariants | deterministic unit tests |
| `T1` | cross-module behavior | integration tests (HTTP/DB/service) |
| `T2` | user-like runtime proof | E2E API/WS + browser path assertions |

## Mandatory Acceptance Pack

| ID | Scenario |
|---|---|
| `API-SETUP-01` | first-run registration lockout behavior |
| `API-AUTH-02` | session cookie issuance |
| `API-NOTE-03` | create + fetch note projection |
| `API-NOTE-04` | stale version conflict (`409`) |
| `API-REC-01` | typed metadata upsert/delete |
| `API-SEARCH-01` | wiki link and backlink search correctness |
| `API-ATT-01` | 500 MB attachment upload path |
| `API-ATT-02` | >500 MB deterministic reject |
| `WS-01..05` | WS subscribe/patch/conflict/replay/idempotency |
| `E2E-01..05` | setup/login/edit/multitab/attachment/rollback/export |

## Determinism Rules

- use bounded timeouts and explicit diagnostics
- avoid unbounded sleeps
- capture request IDs and WS sequence evidence on failures

## Related

- Performance targets: [performance.md](performance.md)
- CI profiles: [/docs/reference/CI.md](/docs/reference/CI.md)
