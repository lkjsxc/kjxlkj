# Current TODO

Active task list for implementation - Iteration 25.

## Active Tasks

| ID | Task | Status |
|----|------|--------|
| 1 | Fix insert mode text input handling | pending |
| 2 | Fix file loading from command line | pending |
| 3 | Run tests and verify fixes | pending |
| 4 | Recreate TODO list | pending |
| 5 | Continue to next iteration | pending |

## Bug Analysis (Iteration 25)

### Bug 1: Insert Mode Key Input
- **Symptom**: Mode switches to Insert, but typed characters not displayed
- **Root Cause**: `CoreTask::process_intent()` has catch-all `_ => {}` that ignores InsertText, Backspace, etc.
- **Fix**: Implement all IntentKind handlers in process_intent()

### Bug 2: File Loading
- **Symptom**: `cargo run ./README.md` shows [No Name] and empty content
- **Root Cause**: 
  1. `args.files` never sent to core task
  2. `Action::OpenFile { path }` does nothing in process_action()
- **Fix**: 
  1. Send OpenFile action for each CLI file
  2. Implement file loading in process_action()

## Related

- Plan: [plan.md](plan.md)
- Completed: [completed.md](completed.md)
