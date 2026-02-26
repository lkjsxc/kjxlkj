# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Mandatory verification contract for reconstruction.

## Verification Tiers

| Tier | Purpose | Required Evidence |
|---|---|---|
| `T0` | local invariants | deterministic unit/property tests |
| `T1` | cross-module behavior | integration tests (HTTP/WS/DB/services) |
| `T2` | user-like proof | browser E2E + API/WS assertions |

All three tiers are mandatory release gates.

## Mandatory Acceptance Pack

| ID | Scenario |
|---|---|
| `API-NOTE-01` | create note without title defaults title to current datetime |
| `API-NOTE-02` | note `id` remains stable while title changes |
| `API-SEARCH-01` | lexical search results and ranking are deterministic |
| `API-SEARCH-02` | semantic search mode works and merges with lexical in hybrid mode |
| `API-SEARCH-03` | embedding-provider outage degrades to lexical mode with diagnostics |
| `API-AUTO-03` | `kjxlkj-agent` rule validates prompt JSON and mode fields |
| `API-AUTO-04` | agent XML instruction parse/retry/fail semantics |
| `WS-04` | duplicate idempotency key returns same commit identity |
| `WS-05` | reconnect + cursor replay is deterministic |
| `WS-06` | agent automation events stream in order |
| `E2E-06` | markdown editor autosave confidence path |
| `E2E-12` | compact top-right menu behavior |
| `E2E-17` | draft integrity under conflicts/reconnect |
| `E2E-19` | `320px` layout remains usable and no horizontal scroll |
| `E2E-23` | create-new-note creates and selects note immediately |
| `E2E-24` | editor supports Obsidian-like markdown workflows |
| `E2E-25` | compact mode activates at `<=1280px` and closes on select |
| `AGENT-01` | prompt fully loaded from JSON and validated |
| `AGENT-02` | KV memory persists across loops and is mutable |
| `AGENT-03` | YOLO mode can create/edit notes inside scope guardrails |
| `AGENT-04` | full conversation transcript retention remains disabled |

## Communication Layer Priority Pack

The following acceptance IDs are high-criticality and MUST pass before any release candidate:

- `API-NOTE-01`
- `API-NOTE-02`
- `API-SEARCH-01`
- `API-SEARCH-02`
- `API-SEARCH-03`
- `WS-04`
- `WS-05`
- `WS-06`
- `API-AUTO-03`
- `API-AUTO-04`

## Determinism Rules

- use bounded timeouts and explicit diagnostics
- avoid unbounded sleeps/retries
- capture request IDs and event sequence evidence on failures
- capture prompt hash and parser version for agent runs

## Required Suite Categories

| Category | Tier | Required Focus |
|---|---|---|
| domain property tests | `T0` | ID immutability, version increments, soft-delete invariants |
| error envelope tests | `T0` | `code/message/details/request_id` shape and status mapping |
| auth/session/csrf tests | `T1` | login/logout, cookie rules, csrf coverage |
| HTTP contract tests | `T1` | endpoint status, idempotency, conflict and rate-limit paths |
| WebSocket protocol tests | `T1` | replay ordering, stale cursor, idempotent patch replay |
| degraded dependency tests | `T1` | embedding/LLM outage fallback behavior |
| end-to-end editor tests | `T2` | autosave, conflict recovery, compact mode behavior |
| multi-client sync tests | `T2` | reconnect, replay, convergence assertions |

## Evidence Contract

- Every acceptance ID MUST map to one primary test suite in [/docs/reference/TEST_MATRIX.md](/docs/reference/TEST_MATRIX.md).
- Every completed wave MUST append evidence links in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md).
- No TODO checkbox may be marked complete when mapped acceptance IDs lack evidence.

## Related

- UX requirements: [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- CI profiles: [/docs/reference/CI.md](/docs/reference/CI.md)
- Error model: [/docs/spec/api/errors.md](/docs/spec/api/errors.md)
