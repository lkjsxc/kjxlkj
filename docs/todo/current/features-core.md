# TODO: Features (Core)

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Terminal

- [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
- [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- [/docs/spec/features/terminal/escape-parser.md](/docs/spec/features/terminal/escape-parser.md)

### Terminal emulator

- [x] Screen buffer: cell model (grapheme, width, fg, bg, attrs, is_wide_continuation)
- [x] Escape sequence parser: CUU/CUD/CUF/CUB/CUP cursor movement
- [x] Escape sequence parser: ED/EL/ECH erase operations
- [ ] Escape sequence parser: SU/SD/DECSTBM scroll regions
- [x] Escape sequence parser: SGR (basic 8, bright 8, 256-color, 24-bit RGB)
- [ ] Escape sequence parser: DECSET/DECRST private modes (alt screen, cursor visibility)
- [ ] Escape sequence parser: OSC (window title, clipboard)
- [ ] Scrollback buffer with configurable capacity

### PTY management

- [x] PTY spawn via openpty/forkpty
- [x] Async read from PTY fd
- [x] Write to PTY fd
- [x] Resize via ioctl TIOCSWINSZ + SIGWINCH
- [ ] Cleanup via SIGHUP on close

### Terminal as window

- [x] Terminal windows share WindowId in layout tree
- [ ] Ctrl-w navigation works with terminal panes
- [x] SIGWINCH on terminal pane resize
- [ ] SIGHUP on terminal pane close
- [ ] Terminal Normal mode (scrollback navigation)

### Terminal features

- [ ] tmux integration per [/docs/spec/features/terminal/tmux.md](/docs/spec/features/terminal/tmux.md)
- [ ] DAP debugging per [/docs/spec/features/terminal/dap.md](/docs/spec/features/terminal/dap.md)
- [ ] Remote editing per [/docs/spec/features/terminal/remote.md](/docs/spec/features/terminal/remote.md)
- [ ] WM integration per [/docs/spec/features/terminal/wm-integration.md](/docs/spec/features/terminal/wm-integration.md)

## Session management

- [/docs/spec/features/session/README.md](/docs/spec/features/session/README.md)
- [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)

### Session save/load

- [x] `:SessionSave` writes JSON per session schema
- [x] `:SessionLoad` reads JSON and restores window layout
- [x] Recursive LayoutNode tree (leaf/hsplit/vsplit)
- [x] WindowRef with content_type (buffer/terminal)
- [x] Buffer restoration (path, cursor, viewport)
- [ ] Tab restoration
- [ ] Register/mark persistence

### Session features

- [ ] Auto-save per [/docs/spec/features/session/auto_save.md](/docs/spec/features/session/auto_save.md)
- [ ] Undo tree per [/docs/spec/features/session/undo_tree.md](/docs/spec/features/session/undo_tree.md)
- [ ] View management per [/docs/spec/features/session/view-management.md](/docs/spec/features/session/view-management.md)
- [ ] Workspaces per [/docs/spec/features/session/workspaces.md](/docs/spec/features/session/workspaces.md)
- [ ] Project config per [/docs/spec/features/session/project-config.md](/docs/spec/features/session/project-config.md)
- [ ] Macros per [/docs/spec/features/session/macros.md](/docs/spec/features/session/macros.md)
- [ ] Registers per [/docs/spec/features/session/registers.md](/docs/spec/features/session/registers.md)
- [ ] Expression register per [/docs/spec/features/session/expression-register.md](/docs/spec/features/session/expression-register.md)
- [ ] Ex commands detailed per [/docs/spec/features/session/ex-commands-detailed.md](/docs/spec/features/session/ex-commands-detailed.md)

## Window features

- [/docs/spec/features/window/README.md](/docs/spec/features/window/README.md)
- [ ] Split windows per [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [ ] Splits advanced per [/docs/spec/features/window/splits-advanced.md](/docs/spec/features/window/splits-advanced.md)
- [ ] Floating windows per [/docs/spec/features/window/floating-windows.md](/docs/spec/features/window/floating-windows.md)
- [ ] Tabs per [/docs/spec/features/window/tabs.md](/docs/spec/features/window/tabs.md)
- [ ] Window command per [/docs/spec/features/window/wincmd.md](/docs/spec/features/window/wincmd.md)
- [ ] Window layouts per [/docs/spec/features/window/window-layouts.md](/docs/spec/features/window/window-layouts.md)
- [ ] Window presets per [/docs/spec/features/window/window-presets.md](/docs/spec/features/window/window-presets.md)
- [ ] Window resize per [/docs/spec/features/window/window-resize-modes.md](/docs/spec/features/window/window-resize-modes.md)
- [ ] Window resizer per [/docs/spec/features/window/window_resizer.md](/docs/spec/features/window/window_resizer.md)
- [ ] Window zoom per [/docs/spec/features/window/window-zoom.md](/docs/spec/features/window/window-zoom.md)

## Buffer features

- [/docs/spec/features/buffer/README.md](/docs/spec/features/buffer/README.md)
- [ ] Buffer switching per [/docs/spec/features/buffer/buffer-switching.md](/docs/spec/features/buffer/buffer-switching.md)
- [ ] Buffer groups per [/docs/spec/features/buffer/buffer-groups.md](/docs/spec/features/buffer/buffer-groups.md)
- [ ] Buffer-local options per [/docs/spec/features/buffer/buffer-local-options.md](/docs/spec/features/buffer/buffer-local-options.md)
- [ ] Buffer advanced per [/docs/spec/features/buffer/buffer-advanced.md](/docs/spec/features/buffer/buffer-advanced.md)
- [ ] Bufferline per [/docs/spec/features/buffer/bufferline.md](/docs/spec/features/buffer/bufferline.md)
- [ ] Alternate file per [/docs/spec/features/buffer/alternate-file.md](/docs/spec/features/buffer/alternate-file.md)
- [ ] Arglist per [/docs/spec/features/buffer/arglist.md](/docs/spec/features/buffer/arglist.md)

## Wiring verification

Per [/docs/log/proposals/deep-wiring-checklist.md](/docs/log/proposals/deep-wiring-checklist.md):

- [x] `:terminal` spawns a real PTY process via openpty/forkpty, not a stub
- [x] Terminal screen buffer receives and parses escape sequences from PTY output
- [ ] Typing in terminal-insert mode forwards raw bytes to the PTY fd
- [ ] `Ctrl-\ Ctrl-n` switches terminal to Normal mode for scrollback navigation
- [x] `:SessionSave` serializes the full layout tree to JSON per session schema
- [x] `:SessionLoad` reads JSON and reconstructs window layout with correct cursor positions
- [ ] Window `Ctrl-w h/j/k/l` navigation works identically for buffer and terminal windows
- [x] Terminal window resize triggers ioctl(TIOCSWINSZ) and SIGWINCH delivery
- [x] Buffer `:e {file}` dispatches FileRead to FS service and constructs rope on response
- [x] Buffer `:w` dispatches FileWrite to FS service with rope snapshot
