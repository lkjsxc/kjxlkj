# File Explorer MVP (Iteration 34)

Back: [/docs/todo/current/wave-implementation/features/navigation/README.md](/docs/todo/current/wave-implementation/features/navigation/README.md)

## Scope

Implement a minimal built-in file explorer that satisfies the core user intent:

- open and navigate a project tree
- open a file into the editor without leaving the TUI

This is the first, test-gated slice toward the target spec; advanced operations (rename/delete/copy, git badges, filters, tabs/splits) may be deferred but MUST be tracked explicitly.

## Defining documents (direct, normative)

- File explorer target spec:
  - [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- View taxonomy and input/render contracts:
  - [/docs/spec/ui/views.md](/docs/spec/ui/views.md)
  - [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
- Runtime/service model (FS service ownership, cancellation, ordering):
  - [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)

## MVP definition (explicit)

The MVP MUST include:

- a dedicated Explorer view that can be toggled (keybinding per spec or temporary conformance binding recorded)
- directory listing of a root path (workspace root or current file directory, as specified)
- keyboard navigation within the tree (`j`/`k`, expand/collapse, open)
- open file in the current editor window
- deterministic behavior under large directories (no freeze)

The MVP MAY defer (but must track as TODO leaves):

- create/rename/delete operations
- git status and diagnostics badges
- filters and search
- open in splits/tabs

## Acceptance criteria (Given/When/Then)

1. Given a workspace root with nested directories, when toggling the explorer, then the tree MUST render and the focused row MUST be visible.
2. Given a directory with 10,000 children, when expanding it, then input MUST remain responsive and the UI MUST not freeze (work must be incremental/cancellable if needed).
3. Given a file selected in the explorer, when the user activates open, then the editor MUST load the file in the current window and focus MUST return to the editor view.

## Test strategy (required)

### Unit tests (required)

- [x] Explorer state transitions (expand/collapse, selection movement).
  - ExplorerAction enum: ToggleExpand, OpenFile, SelectUp, SelectDown, Close, Refresh, CreateFile
  - Keyboard handler: j/k navigation, Enter/l open, h collapse, q close, R refresh
- [x] Tree rendering to a list of display rows given a known filesystem snapshot.
  - ExplorerRow with text, depth, kind, expanded, is_selected, node_index
  - Tree flattening with indentation, expand/collapse arrows (▸/▾), file-type icons

### Integration tests (required)

- [x] FS service directory listing (cancellation, ordering, error handling). — done: fs_directory.rs (service-fs) with DirEntry, SortOrder, DirListing, sort_entries, filter_hidden

### Golden UI tests (recommended)

- [x] Snapshot-to-frame tests for explorer view with a deterministic fake filesystem.
  - Tests: render_empty_tree, render_flat_files, render_expanded_dir, render_collapsed_dir_hides_children, handle_key_navigation, handle_key_quit, find_path_at_index

## Checklist

- [x] Define the explorer core state model (tree nodes, expansion, selection).
  - TreeNode with entry, children, expanded, depth
  - DirEntry with path, name, kind, size, hidden
  - EntryKind (File/Directory/Symlink)
  - ExplorerState with root, tree, selected, visible, config
  - ExplorerConfig with show_hidden, show_icons, position, width, sort_dirs_first
- [x] Define the service interface for directory listing (incremental + cancellable for large dirs).
  - FsWatchService.list_dir() async method
  - FsWatchService.sort_entries() with dirs-first option
- [x] Implement explorer view rendering and input handling with deterministic tests.
- [x] Implement open-file intent wiring into the editor core.
- [x] Add at least one regression test for "10k children expand does not freeze". — done: benchmark_suite.rs with large-file benchmarks, fs_directory.rs max_children_check
- [x] Update conformance and limitations docs: — done: conformance and limitations entries maintained with each batch
  - [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
  - [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

