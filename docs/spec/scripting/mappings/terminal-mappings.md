# Terminal Mode Mappings

Back: [docs/spec/scripting/mappings/README.md](/docs/spec/scripting/mappings/README.md)

Mappings active in the terminal emulator window.

## Overview

Terminal mode mappings intercept keys before they
reach the terminal process. This enables editor-level
keybindings to work even when a terminal window is
focused, such as switching windows or entering
normal mode.

## Commands

### Recursive Mapping

`:tmap {lhs} {rhs}` creates a terminal-mode mapping
where the RHS is re-interpreted through mappings.

### Non-Recursive Mapping

`:tnoremap {lhs} {rhs}` creates a terminal-mode
mapping where the RHS is NOT re-interpreted.

### Remove Mapping

`:tunmap {lhs}` removes a terminal-mode mapping.

### Clear All

`:tmapclear` removes all terminal-mode mappings.

## Default Mappings

### Exit Terminal Mode

| Mapping | Effect |
|---------|--------|
| `<C-\><C-n>` | Exit to Normal mode (built-in) |

This is the only built-in terminal-mode mapping.
All other keys pass through to the terminal process.

## Common User Mappings

### Window Navigation

Map `<C-w>` prefix to window commands so users can
switch windows without leaving terminal mode.

| Example Mapping | Effect |
|-----------------|--------|
| `<C-w>h` | Focus window left |
| `<C-w>j` | Focus window below |
| `<C-w>k` | Focus window above |
| `<C-w>l` | Focus window right |

### Quick Escape

Map `<Esc><Esc>` (double escape) to exit terminal
mode. Single `<Esc>` passes through to the terminal
program (important for vim-in-terminal, etc.).

## Key Pass-Through

### Default Behavior

All keys NOT matched by a terminal-mode mapping are
sent directly to the terminal process as raw bytes.
This includes Ctrl combinations, function keys,
and escape sequences.

### Precedence

Terminal-mode mappings are checked before keys are
sent to the terminal pty. If a mapping matches, the
RHS is executed; otherwise the raw key passes through.

## Interaction with Terminal Process

### Shell Key Bindings

Terminal-mode mappings can shadow shell key bindings.
Users must be careful not to map keys that their
shell or TUI application needs (e.g., Ctrl-C, Ctrl-Z).

### TUI Applications

When running TUI applications (vim, htop, etc.)
inside the terminal, mappings may interfere. The
double-escape pattern avoids conflicts since most
programs do not use double-escape.

## Expression Mappings

`:tnoremap <expr> {lhs} {expr}` evaluates an
expression to produce the RHS. Can check terminal
state (running command, title) for conditional
behavior.

## Listing Mappings

`:tmap` with no arguments lists all terminal-mode
mappings. `:verbose tmap` shows definition source.

## Related

- Terminal emulator: [docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)
- Terminal mode: [docs/spec/modes/terminal.md](/docs/spec/features/terminal/terminal.md)
- Mapping modes: [docs/spec/scripting/mappings/mapping-modes.md](/docs/spec/scripting/mappings/mapping-modes.md)
