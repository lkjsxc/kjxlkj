# Multiple Cursors

## User intent

Make repeated edits across multiple locations simultaneously.

## Model

| Concept | Requirement |
|---|---|
| Primary cursor | The cursor that anchors viewport and mode decisions. |
| Secondary cursors | Additional carets that receive mirrored edits. |
| Selection sets | Each cursor may carry a selection range. |
| Conflict policy | Overlapping edits MUST be resolved deterministically. |

## Interaction

| Action | Requirement |
|---|---|
| Add next match | Add a cursor at the next occurrence of the selection. |
| Skip match | Skip the current match and continue. |
| Add arbitrary | Add cursors by search results or manual selection. |

## Determinism requirements

- Applying edits from multiple cursors MUST yield the same result regardless of cursor insertion order.
- The core MUST serialize multi-cursor edits into a single transaction.

## Acceptance criteria

- Multi-cursor operations MUST preserve editor responsiveness.
- Undo MUST revert the entire multi-cursor change as one step.
