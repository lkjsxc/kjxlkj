# Current TODO

Active task list for implementation - Iteration 29.

## Bug Analysis

### Bug 1: Main text is not rendered at all
- **Root Cause**: App loop reads snapshot but command line content is not being sent to parser
- **Issue**: InputParser updates `cmdline` internally but snapshot doesn't include it
- **Fix**: Pass command line state to snapshot, ensure buffer lines are populated

### Bug 2: Key input not accepted in command mode
- **Root Cause**: parse_command returns None for character inputs (updates internal cmdline state)
- **Issue**: Snapshot doesn't reflect command line changes because it's in InputParser not EditorState
- **Fix**: Track command line in EditorState and update via Intent

### Bug 3: File browser cannot be displayed (Space+E)
- **Root Cause**: No Space+E keybinding exists in parser
- **Issue**: File explorer feature not yet implemented
- **Fix**: Add leader key (Space) support and file explorer intent

## Active Tasks

| ID | Task | Status |
|----|------|--------|
| 1 | Update docs for iteration 29 | ✓ complete |
| 2 | Fix command line tracking | pending |
| 3 | Add debug dump mode | pending |
| 4 | Verify and test fixes | pending |
| 5 | Git commit | pending |

## Related

- Plan: [plan.md](plan.md)
- Completed: [completed.md](completed.md)
