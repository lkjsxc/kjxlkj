# Current TODO

Active task list for implementation - Iteration 31 Complete.

## Completed (Iteration 31)

- [x] Fixed cursor not displaying (always show cursor even on empty cells)
- [x] Fixed cursor movement (ensure_cursor_visible after motion)
- [x] Fixed viewport not following cursor
- [x] Fixed command mode retaining previous input (reset prompt on close)
- [x] Implemented file explorer state and rendering
- [x] Added file explorer toggle (Space+e) and focus (Space+E)
- [x] Added tests for file explorer and cmdline
- [x] All 849+ tests passing

## Completed (Iteration 29-30)

- [x] Fixed viewport dimensions for text rendering
- [x] Fixed command line display
- [x] Added --dump CLI option for debugging
- [x] Added leader key (Space) support
- [x] Added file explorer intents (Space+e, Space+E)
- [x] Added terminal toggle intent (Space+t)
- [x] Added operator parsing (d, y, c, >, <)
- [x] Added dd, yy, cc line operation support
- [x] Extracted parser_operators.rs
- [x] All 838+ tests passing

## Bug Status

| Bug | Status |
|-----|--------|
| Main text not rendered | ✓ Fixed |
| Command mode input | ✓ Fixed |
| Cursor not displayed | ✓ Fixed |
| Cursor movement incorrect | ✓ Fixed |
| Viewport not following cursor | ✓ Fixed |
| File explorer | ✓ Implemented |

## Active Tasks

| ID | Task | Status |
|----|------|--------|
| 1 | All tasks complete | ✓ |

## Related

- Plan: [plan.md](plan.md)
- Completed: [completed.md](completed.md)
