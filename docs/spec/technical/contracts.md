# Contracts

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Contracts are invariants that the implementation MUST uphold.
They serve as the boundary conditions for correctness.

## Determinism contract

| Topic | Requirement | Violation consequence |
|---|---|---|
| Edit serialization | All user-visible edits MUST be applied by the core task in a single total order. | Race conditions; corrupted buffer |
| Versioning | Buffers MUST have monotonic versions; service results MUST carry versions. | Stale data applied |
| Stale results | The core MUST discard results that target older versions. | Ghost diagnostics; wrong highlights |
| Undo | Undo/redo MUST operate on the serialized edit stream. | Inconsistent undo history |
| Replay | Given the same edit stream, the buffer state MUST be identical. | Non-deterministic behavior |

## Service contract

| Topic | Requirement | Violation consequence |
|---|---|---|
| Typed messages | Requests and results MUST be typed enums (no unstructured strings). | Deserialization panics |
| Supervision | Services MUST be restartable and report health via heartbeats. | Silent failures |
| Timeouts | Requests MUST carry deadlines; expired requests MUST be dropped. | Blocked core |
| Cancellation | Requests MUST be cancellable; cancellation MUST be idempotent. | Resource leaks |
| Isolation | Service panics MUST NOT propagate to the core task. | Total crash |
| Backpressure | Channels MUST have bounded capacity; senders MUST handle full channels. | OOM |

## Snapshot contract

| Topic | Requirement | Violation consequence |
|---|---|---|
| Read-only | Render MUST consume snapshots without mutation. | Data races |
| Completeness | Snapshots MUST include all state needed for rendering without querying services. | Render stall |
| Coalescing | Rapid snapshot updates MAY be coalesced; last-wins is acceptable. | N/A (performance optimization) |
| Atomicity | A snapshot MUST represent a consistent point in time. | Torn reads |

## Buffer contract

| Topic | Requirement | Violation consequence |
|---|---|---|
| UTF-8 | Buffer text MUST always be valid UTF-8. | Panics in string operations |
| Line index | Line start offsets MUST be updated atomically with text changes. | Wrong cursor position |
| Change notification | Every text mutation MUST emit a change event with old range and new text. | Stale syntax tree |

## Observability contract

| Signal | Requirement | Exposure |
|---|---|---|
| Queue depth | Each channel capacity and current usage MUST be observable. | Statusline, debug log |
| Latency | Request-to-result latency MUST be measurable per service. | Debug log, profiling |
| Errors | Service errors MUST be visible in the diagnostics list. | Quickfix list |
| Memory | Buffer memory usage MUST be queryable. | `:buffers` output |

## Persistence contract

| Topic | Requirement | Violation consequence |
|---|---|---|
| Atomic writes | File saves MUST use write-to-temp + rename to prevent partial writes. | Data loss on crash |
| Backup | Before overwriting, a backup MUST be created if configured. | No recovery option |
| Encoding | File encoding MUST be preserved on save unless explicitly changed. | Mojibake |
