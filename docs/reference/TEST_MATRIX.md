# Test Matrix

Back: [/docs/reference/README.md](/docs/reference/README.md)

Canonical mapping from acceptance IDs to mandatory test suites and evidence owners.

## Rules

- Every acceptance ID in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) MUST appear here.
- Every row MUST have at least one owning TODO wave.
- Release is blocked until all rows are `pass` with evidence links.

## Matrix

| Acceptance ID | Tier | Primary Suite | Focus | Owning Wave | Evidence Target | Status |
|---|---|---|---|---|---|---|
| `API-NOTE-01` | `T1` | `http_notes_contract` | datetime default title | `S02/W020` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `API-NOTE-02` | `T1` | `http_notes_contract` | immutable ID vs mutable title | `S02/W020` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `API-SEARCH-01` | `T1` | `search_contract` | lexical determinism | `S02/W022` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `API-SEARCH-02` | `T1` | `search_contract` | hybrid/semantic behavior | `S02/W022` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `API-SEARCH-03` | `T1` | `search_degradation` | semantic outage fallback | `S02/W022` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `API-AUTO-03` | `T1` | `automation_contract` | agent rule payload validity | `S04/W041` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `API-AUTO-04` | `T1` | `automation_contract` | XML parse/retry/fail behavior | `S04/W040` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `WS-04` | `T1` | `ws_idempotency` | duplicate key replay identity | `S07/W071` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `WS-05` | `T1` | `ws_replay` | deterministic reconnect replay | `S07/W071` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `WS-06` | `T1` | `ws_automation_stream` | ordered automation event stream | `S07/W072` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `E2E-06` | `T2` | `editor_autosave_e2e` | autosave confidence path | `S08/W081` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `E2E-12` | `T2` | `menu_interaction_e2e` | compact top-right menu behavior | `S08/W082` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `E2E-17` | `T2` | `conflict_integrity_e2e` | draft integrity under reconnect/conflict | `S08/W081` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `E2E-19` | `T2` | `mobile_layout_e2e` | 320px usability/no horizontal scroll | `S08/W082` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `E2E-23` | `T2` | `note_create_select_e2e` | immediate create+select flow | `S08/W080` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `E2E-24` | `T2` | `obsidian_flow_e2e` | markdown workflow parity | `S08/W081` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `E2E-25` | `T2` | `compact_threshold_e2e` | compact mode at <=1280px | `S08/W082` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `AGENT-01` | `T1` | `agent_prompt_contract` | prompt JSON load + validation | `S04/W041` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `AGENT-02` | `T1` | `agent_memory_contract` | KV memory persistence/mutability | `S04/W042` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `AGENT-03` | `T1` | `agent_yolo_scope_contract` | scoped YOLO create/edit | `S04/W042` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |
| `AGENT-04` | `T1` | `agent_privacy_contract` | transcript retention disabled | `S04/W040` | [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md) | `pending` |

## Communication-Layer Supplemental Suites

These suites are mandatory for release even when not tied to one acceptance ID.

| Suite | Tier | Purpose | Owning Wave | Status |
|---|---|---|---|---|
| `error_envelope_unit` | `T0` | ensure deterministic error payload shape | `S06/W060` | `pending` |
| `http_rate_limit_integration` | `T1` | verify 429 + Retry-After | `S05/W052` | `pending` |
| `ws_stale_cursor_integration` | `T1` | stale cursor schema and recovery path | `S07/W071` | `pending` |
| `ws_reconnect_storm_chaos` | `T2` | reconnect churn + convergence | `S10/W101` | `pending` |
| `frontend_http_client_contract` | `T1` | verify x-request-id, csrf, idempotency-key, and error-envelope handling in frontend transport | `S08/W080` | `pending` |
| `frontend_ws_replay_contract` | `T1` | verify reconnect replay, ack cursor, and stale-cursor handling in frontend client | `S08/W081` | `pending` |
| `frontend_comm_degradation_e2e` | `T2` | verify offline/retry/degraded communication UX convergence | `S08/W081` | `pending` |
| `frontend_auth_session_rotation` | `T2` | verify setup/login/session-expiry recovery path and draft preservation | `S08/W080` | `pending` |

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- TODO trace map: [TODO_TRACE_MATRIX.md](TODO_TRACE_MATRIX.md)
- CI contract: [CI.md](CI.md)
