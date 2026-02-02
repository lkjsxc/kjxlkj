# Undo Tree

## User intent

Explore and restore past states, including branches.

## Model

| Concept | Requirement |
|---|---|
| Node | Represents an edit transaction. |
| Edge | Parent-child relation; branches represent divergent histories. |
| Cursor | Current node pointer per buffer. |
| Checkpoint | Optional labeled nodes for sessions. |

## UX surface

| Capability | Requirement |
|---|---|
| Visual tree | Render nodes and branches; navigate with keys. |
| Preview | Selecting a node previews metadata (time, summary). |
| Restore | Jumping to a node applies inverse edits as one transaction. |

## Async considerations

Undo operations are local and SHOULD be synchronous.

However:

- Persistence of undo history MAY be async.
- Large undo metadata rendering MUST not block the core.

## Acceptance criteria

- Undo/redo MUST be deterministic and reversible.
- Undo tree UI MUST handle thousands of nodes.
- Session restore MUST not corrupt tree integrity.
