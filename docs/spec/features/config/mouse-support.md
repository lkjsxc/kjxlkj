# Mouse Support (Non-goal)

Back: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)

Mouse interaction is intentionally unsupported. This document specifies the explicit non-support contract.

## Requirements (normative)

| Rule | Requirement |
|---|---|
| Input | Mouse events from the terminal MUST be silently discarded. The editor MUST NOT request mouse tracking (no `CSI ?1000h`, `CSI ?1002h`, `CSI ?1006h` on startup). |
| UX | All essential workflows MUST be keyboard-only. No feature may require mouse interaction. |
| Configuration | There MUST NOT be a configuration option that enables mouse interaction. If a user adds a mouse-related key to their config, the config system MUST emit a diagnostic warning. |
| Keybinding table | Key notation definitions MAY include mouse token constants for parsing compatibility, but those tokens MUST map to no-op handlers. |
| Terminal setup | The startup sequence (`CSI` setup) MUST NOT enable any mouse reporting mode. |
| Terminal emulator | The integrated terminal emulator MUST track mouse mode state from child processes (DECSET 1000/1002/1006) so that DECRST properly clears it, but the editor itself never sends mouse events. |

## Rationale

The keyboard-only invariant exists because:

| Reason | Explanation |
|---|---|
| Consistent input model | All actions have deterministic key sequences. No context-dependent click targets. |
| Terminal portability | Mouse support varies widely across terminal emulators and multiplexers. Keyboard input is universally reliable. |
| Accessibility | Screen readers and alternative input devices work with keyboard-driven editors. |
| Vim compatibility | Traditional vi/Vim workflow is keyboard-centric. Mouse is an optional add-on in Vim, rarely used by expert users. |

## Interaction with terminal emulator

When a child process in the integrated terminal emulator enables mouse reporting (e.g., `htop`, `mc`), the terminal emulator MUST track this mode state internally. However, the editor's input layer MUST NOT forward mouse events to the terminal emulator or any other component. This means TUI programs running inside the integrated terminal that require mouse interaction will not receive mouse events.

## Related

- Keyboard-only principle: [/docs/spec/overview/principles.md](/docs/spec/overview/principles.md)
- Terminal emulator: [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- Policy: [/docs/policy/README.md](/docs/policy/README.md)
