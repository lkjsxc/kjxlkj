# Testing Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Mandatory verification contract for reconstruction.

## Verification Tiers

| Tier | Purpose | Required Evidence |
|---|---|---|
| `T0` | local invariants | deterministic unit/property tests |
| `T1` | cross-module behavior | integration tests (HTTP/WS/DB/services) |
| `T2` | user-like proof | browser E2E + API/WS assertions |

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

## Determinism Rules

- use bounded timeouts and explicit diagnostics
- avoid unbounded sleeps/retries
- capture request IDs and event sequence evidence on failures
- capture prompt hash and parser version for agent runs

## Related

- UX requirements: [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- CI profiles: [/docs/reference/CI.md](/docs/reference/CI.md)
