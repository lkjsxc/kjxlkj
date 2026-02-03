# Mouse Support (Non-goal)

Mouse interaction is intentionally unsupported.

## Requirements

| Rule | Requirement |
|------|-------------|
| Input | Mouse input MUST be ignored. |
| UX | All essential workflows MUST be keyboard-only. |
| Configuration | There MUST NOT be a configuration option that enables mouse interaction. |
| Diagnostics | If users configure unknown mouse-related keys, the configuration system MUST surface a visible diagnostic. |

## Notes

- Key notation MAY include mouse tokens for compatibility with existing Vim ecosystems, but those tokens MUST NOT trigger actions in kjxlkj.
- This policy is consistent with the keyboard-only invariant in `/docs/policy/README.md`.
