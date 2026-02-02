# Testing

## Strategy

| Layer | Focus |
|---|---|
| Core unit tests | Pure edits, mode transitions, undo determinism. |
| Service unit tests | Protocol parsing, ranking logic, caching. |
| Integration tests | Message ordering, cancellation, backpressure behavior. |
| Golden UI tests | Snapshot-to-frame stability for critical views. |

## Async correctness requirements

| Concern | Requirement |
|---|---|
| Cancellation | Tests MUST assert that cancelled requests do not mutate visible state. |
| Staleness | Tests MUST ensure stale results are discarded by version checks. |
| Backpressure | Tests MUST ensure overload is visible and does not crash. |
| Recovery | Tests MUST cover service restart and continued editing. |

## Determinism checks

Given identical input streams, the core MUST yield identical serialized edit logs and identical snapshots (ignoring wall-clock timestamps).
