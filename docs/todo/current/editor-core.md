# TODO: Editor Core

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Defining specs

- [/docs/spec/editor/README.md](/docs/spec/editor/README.md)
- [/docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)
- [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)

## Buffers

- [x] Buffer model: id, path, content (rope), modified flag, read-only flag
- [x] Buffer creation from file path (`:e`)
- [x] Buffer creation as empty scratch buffer
- [x] Buffer listing (`:ls`, `:buffers`)
- [x] Buffer switching (`:b`, `:bnext`, `:bprev`)
- [x] Buffer deletion (`:bd`, `:bw`)
- [x] Alternate file (`Ctrl-^`)
- [x] Modified buffer guard (prevent quit without save)

## Windows

- [x] Window model: `WindowId`, content enum (`Buffer(BufferId)` | `Terminal(TerminalId)`)
- [x] Layout tree: leaf, hsplit, vsplit with weights
- [x] Window creation via `:split`, `:vsplit`
- [x] Window navigation: `Ctrl-w h/j/k/l`, `Ctrl-w w`
- [x] Window close: `Ctrl-w c`, `Ctrl-w q`
- [ ] Window resize: `Ctrl-w +/-`, `Ctrl-w </>`, `Ctrl-w =`
- [ ] Window zoom: `Ctrl-w _`, `Ctrl-w |`
- [x] Window rotate: `Ctrl-w r`, `Ctrl-w R`
- [x] Window move: `Ctrl-w H/J/K/L`
- [x] Focus uniqueness invariant: exactly one focused window at all times
- [ ] No overlap, full coverage of terminal area

## Related feature specs

- [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [/docs/spec/features/window/splits-advanced.md](/docs/spec/features/window/splits-advanced.md)
- [/docs/spec/features/window/floating-windows.md](/docs/spec/features/window/floating-windows.md)
- [/docs/spec/features/window/tabs.md](/docs/spec/features/window/tabs.md)
- [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md)
- [/docs/spec/features/window/window-layouts.md](/docs/spec/features/window/window-layouts.md)
- [/docs/spec/features/window/window-presets.md](/docs/spec/features/window/window-presets.md)
- [/docs/spec/features/window/window-resize-modes.md](/docs/spec/features/window/window-resize-modes.md)
- [/docs/spec/features/window/window_resizer.md](/docs/spec/features/window/window_resizer.md)
- [/docs/spec/features/window/window-zoom.md](/docs/spec/features/window/window-zoom.md)

## Wiring verification

Per [/docs/log/proposals/deep-wiring-checklist.md](/docs/log/proposals/deep-wiring-checklist.md):

- [x] Buffer creation from `:e {file}` reads file from disk via FS service, constructs rope
- [x] Buffer `:w` writes rope content to disk via FS service, clears modified flag
- [x] Buffer switching `:b {name}` changes the active window content to the target buffer
- [x] Buffer deletion `:bd` removes buffer from list, closes all windows showing it
- [x] Window split `:split`/`:vsplit` divides current window, shares same buffer
- [x] Window close `Ctrl-w c` removes window, rebalances layout tree
- [x] Window navigation `Ctrl-w h/j/k/l` dispatches from keybinding through core state to focus change
- [x] Viewport follow triggers after every cursor motion and produces correct `top_line`/`left_col`
- [x] Terminal windows created via `:terminal` are leaf nodes in the layout tree
