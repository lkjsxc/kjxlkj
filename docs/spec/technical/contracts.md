# Contracts

## Determinism contract

| Topic | Requirement |
|---|---|
| Edit serialization | All user-visible edits MUST be applied by the core task in a single total order. |
| Versioning | Buffers MUST have monotonic versions; service results must carry versions. |
| Stale results | The core MUST discard results that target older versions. |
| Undo | Undo/redo MUST operate on the serialized edit stream. |

## Service contract

| Topic | Requirement |
|---|---|
| Typed messages | Requests and results MUST be typed (no unstructured strings as protocol). |
| Supervision | Services MUST be restartable and report health. |
| Timeouts | Requests MUST allow time budgets and deadlines. |
| Cancellation | Requests MUST be cancellable; cancellation MUST be idempotent. |
| Isolation | Service failures MUST not corrupt core state. |

## Snapshot contract

| Topic | Requirement |
|---|---|
| Read-only | Render consumes snapshots only. |
| Completeness | Snapshots MUST include enough state to render without querying services. |
| Coalescing | Rapid snapshot updates MAY be coalesced; last-wins is acceptable. |

## Observability contract

| Signal | Requirement |
|---|---|
| Queue depth | Each channel capacity and current usage MUST be observable. |
| Latency | Request-to-result latency MUST be measurable. |
| Errors | Service errors MUST be visible in the diagnostics list. |
