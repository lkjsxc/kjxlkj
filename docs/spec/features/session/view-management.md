# View Management

Save and restore per-window view states.

## Overview

A view captures window-local state (cursor position, scroll offset, folds, local options) so it can be restored when returning to a buffer. This is orthogonal to session management, which captures the full workspace layout.

## View State Contents (normative)

| Element | Saved | Notes |
|---|---|---|
| Cursor line and column | Yes | Absolute position in buffer |
| Scroll/viewport offset | Yes | `topline` value |
| Manual folds | Yes | Fold open/close state |
| Fold options (`foldmethod`, `foldlevel`) | Yes | Window-local copies |
| Local options (`wrap`, `number`, etc.) | Yes | Only window-local overrides |
| Local mappings | Optional | Controlled by `viewoptions` |

Elements NOT saved in views (handled by session or buffer):

| Element | Reason |
|---|---|
| Window size | Session layout handles this |
| Buffer content | File on disk handles this |
| Global options | Config file handles this |

## Automatic View Restore

When a buffer is re-displayed in a window:

1. If a saved view exists for this buffer, restore cursor position and scroll offset.
2. If no view exists, place cursor at the position stored in the `"` mark (last exit position).
3. If neither exists, place cursor at line 1, column 0.

The `"` mark is set automatically when leaving a buffer.

## View Storage

Views are stored as part of session JSON. Each window entry in the session file includes view state for its current buffer.

## Cursor Restoration Details

After restoring cursor position, if the line no longer exists (file was truncated externally), clamp to the last line. If the column no longer exists, clamp to the last column of the line.

## Scroll Restoration

Restore `topline` (first visible line). If the restored `topline` would place the cursor off-screen, adjust `topline` so the cursor is visible (respecting `scrolloff`).

## Fold Persistence

| Fold method | Persistent | Notes |
|---|---|---|
| manual | Via view save | Open/close states stored |
| marker | Automatic | Markers are in file content |
| indent | Automatic | Recalculated from indentation |
| expr | Not persistent | Expression re-evaluated on load |
| syntax | Not persistent | Recalculated from syntax |

## Related

- Session management: [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
- Windows: [/docs/spec/editor/windows.md](/docs/spec/editor/windows.md)
