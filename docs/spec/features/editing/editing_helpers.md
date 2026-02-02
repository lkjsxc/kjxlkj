# Editing Helpers (autopairs/surround/comment class)

## Scope

This feature provides built-in equivalents of common editing helpers:

- Auto-pairs insertion and overtype behavior
- Surround add/change/delete
- Comment/uncomment linewise and blockwise

## Design principle

These helpers are not “UI features”; they are **edit primitives** that:

- Execute as serialized core edits
- Participate in undo as single transactions
- Are mode-aware

## Auto-pairs

| Behavior | Requirement |
|---|---|
| Pair insertion | Typing an opening delimiter inserts the matching closer. |
| Skip closer | Typing a closer at an identical closer advances cursor. |
| Newline rule | Enter inside pairs MAY auto-indent and place closers. |
| Language sensitivity | Pair rules SHOULD vary by language/syntax context. |

## Surround

| Operation | Requirement |
|---|---|
| Add | Wrap selection or text-object with a surround pair. |
| Change | Replace existing surround with another. |
| Delete | Remove the surround while preserving inner text. |

## Comment

| Operation | Requirement |
|---|---|
| Toggle | Toggle comment state for selection/range. |
| Respect syntax | Prefer language comment styles (line vs block). |
| Preserve formatting | Indentation SHOULD remain stable after toggling. |

## Acceptance criteria

- All helper operations MUST undo/redo as single steps.
- Helpers MUST not block; any language-aware lookup must be cached or async.
- When syntax context is unavailable, helpers MUST fall back to conservative defaults.
