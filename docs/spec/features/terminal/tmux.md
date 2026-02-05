# Terminal Multiplexer Integration (tmux reference)

Back: [/docs/spec/features/terminal/README.md](/docs/spec/features/terminal/README.md)

This document specifies how kjxlkj interoperates with an **external terminal multiplexer**.

The editor’s own window/tabs system is an internal multiplexer for splits + embedded terminals; a terminal multiplexer adds capabilities that a single TUI process cannot provide reliably (multi-client attach/detach, “virtual displays”, persistent workspaces).

## Required multiplexer capabilities (workflow contract)

The supported terminal multiplexer MUST provide:

| Capability | Meaning in practice | tmux concept (reference) |
|---|---|---|
| Freely change layouts | Split panes, resize, and apply preset layouts quickly | panes + layouts |
| Multiple editor screens and terminals | Run multiple `kjxlkj` instances and multiple shells concurrently | windows + panes |
| Tabs like a web browser | Named, reorderable, closable tabs; fast switching | windows |
| Virtual displays | Separate workspaces that can be switched and persisted | sessions |
| Attach/detach | Resume work from a different terminal later | clients + sessions |

## Supported multiplexers

| Multiplexer | Status | Notes |
|---|---|---|
| tmux | Reference implementation | This document uses tmux terminology for concreteness. |
| GNU screen | Compatible subset | May lack modern ergonomics; treat as “best effort”. |
| WezTerm | Supported | Provides tabs/panes and “workspaces” similar to sessions. |

## Key conflict policy (normative)

- tmux prefix MUST NOT be `Space` (reserved as kjxlkj `<leader>` by default).
- tmux prefix SHOULD NOT be `Ctrl-w` (reserved by kjxlkj for window commands).
- tmux prefix SHOULD remain `Ctrl-b` unless the user has a strong reason to change it.
- When conflicts exist, prefer changing tmux bindings over changing kjxlkj core navigation/editing keys.

## Terminal compatibility requirements (normative)

When kjxlkj is run inside a multiplexer:

| Concern | Requirement |
|---|---|
| Escape/meta latency | Multiplexer configuration SHOULD avoid large `escape-time` delays so `Esc` and Alt/Meta chords do not feel laggy. |
| True color | The environment SHOULD provide 24-bit color so themes render correctly. |
| Cursor shape | Cursor shape changes (Normal/Insert/Replace) SHOULD remain visible; if not possible, the editor MUST still be usable. |
| Clipboard | OSC52 copy SHOULD work through the multiplexer to enable remote/persisted clipboard flows. |
| Focus/resize events | Focus and resize events SHOULD be delivered; missing focus MUST NOT break correctness. |
| No mouse support | Mouse input is ignored by kjxlkj by policy; multiplexer mouse features MUST NOT be required for usability. |

## Workflow patterns (recommended)

### Sessions as “virtual displays”

Use one multiplexer session per project or context:

- session name encodes project + purpose
- detaching/attaching is the primary “move this workspace elsewhere” action
- switching sessions is the primary “switch virtual display” action

### Windows as “tabs”

Use windows as browser-like tabs:

- name windows by task (edit, test, logs, repl)
- reorder windows to keep the current focus area left-to-right
- keep one window for “always-on” shells (build/test)

### Panes as layouts

Use panes for short-lived layout changes:

- split and resize freely during a task
- collapse back to a single pane when done
- prefer multiplexer panes for multiple editor processes; prefer kjxlkj splits for multiple views inside one editor process

## Nested usage

If the integrated terminal runs a shell inside kjxlkj, that shell MAY also run tmux.

When nesting:

- distinguish keybinding layers (tmux prefix vs kjxlkj `<leader>` vs terminal-mode keys)
- avoid “prefix inside prefix” confusion by using visual cues (statusline/tabline) and consistent naming

## Testing requirements (target)

In addition to the PTY E2E tests required by [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md), the project SHOULD include a “multiplexer smoke” PTY E2E that:

- launches a tmux session
- runs kjxlkj inside it
- performs a minimal edit + `:wq` flow

This detects environment-sensitive regressions (escape-time, key normalization, focus/resize routing).
