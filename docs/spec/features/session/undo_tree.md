# Undo Tree

Back: [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)

## User intent

Explore and restore past states, including branches.

## Model (normative)

| Concept | Type | Requirement |
|---|---|---|
| Node | struct | Represents an edit transaction (one or more text changes) |
| Edge | parent-child | Branches represent divergent histories after undo + new edit |
| Cursor | node ID | Current position pointer per buffer |
| Checkpoint | labeled node | Optional labeled nodes for session save/restore |
| Timestamp | `SystemTime` | Each node records when the edit was made |
| Sequence number | `u64` | Monotonic counter for ordering |

## Data structure (normative)

The undo tree is a rooted, directed tree (not a linked list):

| Property | Value |
|---|---|
| Root | The initial empty-buffer state |
| Children | Ordered list; the last-used child is the "preferred" branch |
| Current node | The node representing the current buffer state |
| Branch count | Unbounded (grows when edits follow undo) |
| Node payload | `Vec<TextChange>` where `TextChange` = `{ range, old_text, new_text }` |

## Navigation commands (normative)

| Command | Action |
|---|---|
| `u` | Move to parent node (undo one transaction) |
| `Ctrl-R` | Move to preferred child (redo) |
| `g-` | Move to the chronologically earlier node (by timestamp) |
| `g+` | Move to the chronologically later node (by timestamp) |
| `:earlier {time}` | Jump to state as of `{time}` ago (e.g., `:earlier 5m`) |
| `:later {time}` | Jump forward by `{time}` |
| `:undolist` | Show all undo branches and their leaf nodes |

## UX surface (normative)

| Capability | Requirement |
|---|---|
| Visual tree | Render nodes and branches in a side panel; navigate with `j`/`k` |
| Preview | Selecting a node previews metadata (time, summary, change count) |
| Restore | Jumping to a node applies inverse/forward edits as one transaction |
| Diff | Optionally show diff between current and selected node |

## Persistence (normative)

| Aspect | Requirement |
|---|---|
| Format | Binary format stored in `undodir` (one file per buffer, named by hash) |
| Save trigger | On buffer write, and periodically (configurable interval) |
| Load | On buffer open, if undo file exists and header matches |
| Integrity | Header includes buffer hash; mismatched files MUST be rejected |
| Cleanup | Undo files older than `undolevels_persist_days` (default: 90) are pruned |

## Async considerations

Undo operations are local and MUST be synchronous (no async boundaries between undo and buffer state).

However:

- Persistence of undo history (save/load) MAY be async.
- Large undo tree rendering MUST not block the core (render from snapshot).

## Acceptance criteria

- Undo/redo MUST be deterministic and reversible.
- Undo tree UI MUST handle thousands of nodes without lag.
- Session restore MUST not corrupt tree integrity.
- Time-based navigation MUST find the correct node within O(n) of total nodes.
- Undo after undo MUST create a new branch, preserving the old redo chain.
