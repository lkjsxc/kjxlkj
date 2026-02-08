# Glossary

Terminology used in kjxlkj specifications. All terms are normative: implementations MUST use these names in type definitions, variable names, and API boundaries.

## Architecture terms

| Term | Meaning |
|------|---------|
| Core task | The single Tokio task that owns and mutates editor state. All editing mutations are serialized here. |
| Service | A supervised Tokio task that performs IO or compute (LSP, Git, FS, Terminal, Index). |
| Snapshot | Immutable read-only projection of editor state sent to the render task via `watch` channel. |
| Message bus | The collection of typed, bounded `mpsc` channels connecting core and services. Not a single shared bus. |
| Stale result | A service result targeting an older `BufferVersion` than the current one; it MUST be discarded by core. |
| Backpressure | Bounded queues with overflow strategies: drop-oldest for snapshots, block-sender for edits. |
| Action | An enum value representing a user intent, produced by the input reader and consumed by the core task. |
| EditorSnapshot | The struct sent from core to render; contains all state needed to draw a frame. |

## Data structure terms

| Term | Meaning |
|------|---------|
| Rope | Persistent data structure for text storage. Supports efficient insert, delete, and index operations on large files. |
| BufferVersion | Monotonic `u64` counter incremented on every buffer mutation. Used for staleness detection. |
| CellGrid | 2D array of `Cell` values representing one frame of terminal output. |
| Cell | A single terminal cell: grapheme, display width (0/1/2), fg color, bg color, attribute bitfield. |
| Continuation cell | A cell with width=0 that occupies the second column of a width-2 (CJK) grapheme. |
| Grapheme cluster | A user-perceived character as defined by Unicode UAX #29. The atomic unit of cursor movement. |

## Editor terms

| Term | Meaning |
|------|---------|
| Buffer | In-memory text container backed by a rope. Each buffer has a unique `BufferId`. |
| Window | A rectangular viewport into a buffer. Multiple windows may view the same buffer. |
| Tab page | A collection of windows arranged in a layout tree. |
| Layout tree | A recursive binary tree of horizontal/vertical splits defining window arrangement within a tab. |
| Mode | Input context determining how key events are interpreted: Normal, Insert, Visual, Command, Replace, OperatorPending, TerminalInsert, TerminalNormal. |
| Motion | A command that moves the cursor and/or defines a range for an operator. |
| Operator | A command that transforms text over a range defined by a motion or text object. |
| Text object | A semantic region selector (word, sentence, paragraph, bracket pair, etc.) used with operators. |
| Register | A named text storage slot for yank, delete, and macro content. |
| Mark | A saved (line, column) position within a buffer, identified by a single character. |
| Desired column | The remembered display column when moving vertically; restored when the target line is long enough. |
| Scrolloff | The minimum number of lines between the cursor and the top/bottom edge of the window. |
| Shiftwidth | The number of columns for one level of indentation. |

## UI terms

| Term | Meaning |
|------|---------|
| Gutter | The left margin of a buffer window: line numbers, sign column, fold column. |
| Statusline | The bottom row of each window showing mode, filename, cursor position, etc. |
| Tab line | The top row of the terminal (when multiple tabs exist) showing tab labels. |
| Command line | The bottom row of the terminal used for `:` commands, `/` search, and messages. |
| Wildmenu | The horizontal completion menu shown in the command-line area. |
| Overlay | A floating panel (completion popup, hover, picker) drawn on top of buffer windows. |
| Finder | A fuzzy-search interface for files, symbols, commands, etc. |

## Related

- Full glossary: [/docs/overview/glossary.md](/docs/overview/glossary.md)
- Architecture: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
