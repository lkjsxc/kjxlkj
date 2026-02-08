# TODO: UI and Rendering

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Defining specs

- [/docs/spec/ui/README.md](/docs/spec/ui/README.md)
- [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
- [/docs/spec/ui/views.md](/docs/spec/ui/views.md)
- [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)

## Render pipeline

- [x] Snapshot-driven rendering: immutable state snapshots from core
- [x] Cell grid computation from buffer content + viewport
- [x] Gutter rendering (line numbers, signs, folds)
- [x] Statusline rendering
- [ ] CJK-aware cell rendering (width-2 graphemes span two cells)
- [x] Diff display (gitsigns, inline diff markers)

## Viewport rendering

- [x] Vertical follow: keep cursor within viewport bounds
- [x] Horizontal follow: scroll to keep cursor column visible
- [ ] Line wrapping algorithm per [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [ ] CJK wrap-boundary padding cell insertion
- [x] Scrolloff/sidescrolloff handling

## Component model

- [x] Component taxonomy per [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
- [x] Render/input contracts for each component type
- [x] Component lifecycle (create, render, destroy)

## View system

- [x] View types per [/docs/spec/ui/views.md](/docs/spec/ui/views.md)
- [x] Workspace topology
- [x] View-window relationship

## Theming

- [x] Theme system per [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)
- [x] Highlight group resolution
- [x] Color scheme loading

## Wiring verification

Per [/docs/log/proposals/deep-wiring-checklist.md](/docs/log/proposals/deep-wiring-checklist.md):

- [x] Core task publishes EditorSnapshot on watch channel after every state change
- [x] Render task receives snapshot and builds cell grid for every visible window
- [x] Cell grid includes correct grapheme, width, fg, bg, attrs for each cell
- [ ] CJK width-2 graphemes produce continuation cells marked `is_wide_continuation`
- [x] Diff rendering compares current grid with previous and emits only changed cells
- [x] All terminal output is flushed in a single `write_all` syscall per frame
- [ ] Cursor highlight spans full display width of the grapheme under cursor
- [ ] Wrap algorithm produces padding cells at CJK boundary, never split wide chars
- [x] Gutter renders line numbers with correct alignment and sign/fold columns
- [x] Statusline updates on every mode change, cursor move, and buffer switch
