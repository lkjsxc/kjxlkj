# TODO: UI and Rendering

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Defining specs

- [/docs/spec/ui/README.md](/docs/spec/ui/README.md)
- [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
- [/docs/spec/ui/views.md](/docs/spec/ui/views.md)
- [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)

## Render pipeline

- [ ] Snapshot-driven rendering: immutable state snapshots from core
- [ ] Cell grid computation from buffer content + viewport
- [ ] Gutter rendering (line numbers, signs, folds)
- [ ] Statusline rendering
- [ ] CJK-aware cell rendering (width-2 graphemes span two cells)
- [ ] Diff display (gitsigns, inline diff markers)

## Viewport rendering

- [ ] Vertical follow: keep cursor within viewport bounds
- [ ] Horizontal follow: scroll to keep cursor column visible
- [ ] Line wrapping algorithm per [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- [ ] CJK wrap-boundary padding cell insertion
- [ ] Scrolloff/sidescrolloff handling

## Component model

- [ ] Component taxonomy per [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
- [ ] Render/input contracts for each component type
- [ ] Component lifecycle (create, render, destroy)

## View system

- [ ] View types per [/docs/spec/ui/views.md](/docs/spec/ui/views.md)
- [ ] Workspace topology
- [ ] View-window relationship

## Theming

- [ ] Theme system per [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)
- [ ] Highlight group resolution
- [ ] Color scheme loading

## Wiring verification

Per [/docs/log/proposals/deep-wiring-checklist.md](/docs/log/proposals/deep-wiring-checklist.md):

- [ ] Core task publishes EditorSnapshot on watch channel after every state change
- [ ] Render task receives snapshot and builds cell grid for every visible window
- [ ] Cell grid includes correct grapheme, width, fg, bg, attrs for each cell
- [ ] CJK width-2 graphemes produce continuation cells marked `is_wide_continuation`
- [ ] Diff rendering compares current grid with previous and emits only changed cells
- [ ] All terminal output is flushed in a single `write_all` syscall per frame
- [ ] Cursor highlight spans full display width of the grapheme under cursor
- [ ] Wrap algorithm produces padding cells at CJK boundary, never split wide chars
- [ ] Gutter renders line numbers with correct alignment and sign/fold columns
- [ ] Statusline updates on every mode change, cursor move, and buffer switch
