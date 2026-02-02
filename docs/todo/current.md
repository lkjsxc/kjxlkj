# Current TODO

Active task list for implementation - Iteration 25.

## Active Tasks

| ID | Task | Status |
|----|------|--------|
| 1 | All bugs fixed | ✓ complete |

## Completed (Iteration 25)

- [x] Analyzed insert mode bug - process_intent() ignored most IntentKind variants
- [x] Created intent_handler.rs with proper intent processing
- [x] Implemented InsertText, InsertNewline, Backspace, DeleteChar handlers
- [x] Analyzed file loading bug - Action::OpenFile was stub, args.files unused
- [x] Implemented CoreTask::open_file() to load file contents
- [x] Updated app.rs to send OpenFile actions for CLI file arguments
- [x] All tests passing

## Bug Analysis (Iteration 25)

### Bug 1: Insert Mode Key Input - FIXED
- **Symptom**: Mode switches to Insert, but typed characters not displayed
- **Root Cause**: `CoreTask::process_intent()` had catch-all `_ => {}` that ignored InsertText
- **Fix**: Created intent_handler.rs with proper handlers for all editing intents

### Bug 2: File Loading - FIXED  
- **Symptom**: `cargo run ./README.md` shows [No Name] and empty content
- **Root Cause**: 
  1. `args.files` never sent to core task
  2. `Action::OpenFile { path }` was a stub
- **Fix**: 
  1. App::run() now sends OpenFile actions for CLI files
  2. CoreTask::open_file() loads file content into buffer

## Related

- Plan: [plan.md](plan.md)
- Completed: [completed.md](completed.md)
