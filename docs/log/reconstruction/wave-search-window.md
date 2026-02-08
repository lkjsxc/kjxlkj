# Reconstruction Wave: Search + Window + WriteAll

## Date
Session continuation.

## Changes Made

### File Splitting (200-line compliance)
- Extracted tests from 7 files: buffer_content.rs, escape_parser.rs, command.rs, visual.rs, buffer.rs, operator_exec.rs, word.rs
- Split action.rs: extracted CommandKind/InsertPosition to action_sub.rs, test to action_tests.rs (242→196 lines)
- Split tree.rs: extracted UndoEntry/UndoGroup to tree_types.rs, timestamp to tree_helpers.rs (229→199 lines)
- All source files now at or under 200 lines. Only dispatch_tests.rs (273, test file) exceeds.

### New Features Implemented
1. **Search** (search.rs, editor_window_ops.rs): Case-insensitive substring search with grapheme-aware indexing. SearchForward, SearchBackward, NextMatch, PrevMatch all wired through dispatch.
2. **Window navigation** (editor_window_ops.rs): FocusWindow(Direction), CycleWindow, CloseWindow. Simplified model using HashMap cycling (no spatial layout tree yet).
3. **Write All** (editor_window_ops.rs): do_write_all() iterates all buffers, do_write_all + quit handled. `:wa` and `:wqa` commands added.
4. **Replace mode entry**: dispatch sets Mode::Replace. Character overwrite handler (do_replace_char_at_cursor) implemented.

### Test Count
217 tests, 0 failures (gained 16 from previous 201).

## Files Created
- search.rs: SearchState with pattern matching and cursor jumping
- editor_window_ops.rs: Window nav, write all, search dispatch, replace mode
- feature_tests.rs: 13 integration tests for new features
- buffer_content_tests.rs, escape_parser_tests.rs, command_tests.rs, visual_tests.rs, buffer_tests.rs, operator_tests.rs, word_tests.rs: Extracted test modules
- action_sub.rs, action_tests.rs: Split from action.rs
- tree_types.rs, tree_helpers.rs: Split from tree.rs

## Remaining Oversized Files
- dispatch_tests.rs (273) — test file, acceptable exception

## TODO Items Completed This Session
- [x] Search motions: `/`, `?`, `n`, `N`
- [x] Forward search, backward search
- [x] Insert-mode navigation (arrow keys) — already existed
- [x] `:wa`, `:wqa` commands
- [x] Window navigation: Ctrl-w h/j/k/l, Ctrl-w w
- [x] Window close: Ctrl-w c/q
- [x] Replace mode entry (R)
