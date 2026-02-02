# Keybinding Hints (which-key class)

## User intent

Discover key sequences without leaving flow.

## Behavior

| Trigger | Requirement |
|---|---|
| Leader prefix | After the leader key, show possible continuations. |
| Timeout | Show hints after a configurable delay. |
| Context | Hints depend on mode and focused view. |
| Search | Allow filtering commands by typing. |

## Data model

| Entity | Meaning |
|---|---|
| `KeymapTrie` | Maps prefixes to commands and sub-prefixes. |
| `HintEntry` | Label, command id, next keys. |
| `Context` | Mode, view, selection state. |

## Acceptance criteria

- Hints MUST never block input; they are derived from in-memory keymaps.
- The UI MUST clearly indicate partial sequences vs complete commands.
