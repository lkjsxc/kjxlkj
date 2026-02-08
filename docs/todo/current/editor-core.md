# TODO: Editor Core

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Defining specs

- [/docs/spec/editor/README.md](/docs/spec/editor/README.md)
- [/docs/spec/editor/buffers.md](/docs/spec/editor/buffers.md)
- [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)

## Buffers

- [ ] Buffer model: id, path, content (rope), modified flag, read-only flag
- [ ] Buffer creation from file path (`:e`)
- [ ] Buffer creation as empty scratch buffer
- [ ] Buffer listing (`:ls`, `:buffers`)
- [ ] Buffer switching (`:b`, `:bnext`, `:bprev`)
- [ ] Buffer deletion (`:bd`, `:bw`)
- [ ] Alternate file (`Ctrl-^`)
- [ ] Modified buffer guard (prevent quit without save)

## Windows

- [ ] Window model: `WindowId`, content enum (`Buffer(BufferId)` | `Terminal(TerminalId)`)
- [ ] Layout tree: leaf, hsplit, vsplit with weights
- [ ] Window creation via `:split`, `:vsplit`
- [ ] Window navigation: `Ctrl-w h/j/k/l`, `Ctrl-w w`
- [ ] Window close: `Ctrl-w c`, `Ctrl-w q`
- [ ] Window resize: `Ctrl-w +/-`, `Ctrl-w </>`, `Ctrl-w =`
- [ ] Window zoom: `Ctrl-w _`, `Ctrl-w |`
- [ ] Window rotate: `Ctrl-w r`, `Ctrl-w R`
- [ ] Window move: `Ctrl-w H/J/K/L`
- [ ] Focus uniqueness invariant: exactly one focused window at all times
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

- [ ] Buffer creation from `:e {file}` reads file from disk via FS service, constructs rope
- [ ] Buffer `:w` writes rope content to disk via FS service, clears modified flag
- [ ] Buffer switching `:b {name}` changes the active window content to the target buffer
- [ ] Buffer deletion `:bd` removes buffer from list, closes all windows showing it
- [ ] Window split `:split`/`:vsplit` divides current window, shares same buffer
- [ ] Window close `Ctrl-w c` removes window, rebalances layout tree
- [ ] Window navigation `Ctrl-w h/j/k/l` dispatches from keybinding through core state to focus change
- [ ] Viewport follow triggers after every cursor motion and produces correct `top_line`/`left_col`
- [ ] Terminal windows created via `:terminal` are leaf nodes in the layout tree
