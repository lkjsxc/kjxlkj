# 2026-02-03: Session Continuation

Back: [/docs/log/2026/02/README.md](/docs/log/2026/02/README.md)

## Summary

Continued implementation of core editing features: search, visual mode operators, find char motions, dot repeat, marks, and substitute command.

## Commits This Session

### feat: implement search functionality (/, ?, n, N)
- Add SearchForward and SearchBackward modes
- Implement forward and backward pattern search with wrap-around
- Add n/N for repeating search in same/opposite direction
- Store search pattern for reuse
- Add status messages showing current pattern and wrap status
- Add 5 tests for search functionality

### feat: implement visual mode operators (d, y, c)
- Add visual selection anchor tracking in EditorState
- Implement VisualDelete, VisualYank, VisualChange actions
- Add motion support in visual mode (hjkl, w/b/e, 0/^/$, gg/G)
- Handle charwise and linewise selection for operators
- Add set_mode method to ModeHandler for mode transitions
- Add set_yank_register method to Buffer
- Add 4 tests for visual mode operations

### feat: implement find char motions (f/t/F/T) and repeat (;/,)
- Add FindCharForward, FindCharBackward, TillCharForward, TillCharBackward motions
- Add RepeatFindChar and RepeatFindCharReverse actions
- Implement cursor movement methods in CursorOps trait
- Track last find char command for repeat functionality
- Support f/t/F/T as operator motions (df{char}, ct{char}, etc.)
- Add 4 tests for find char functionality

### feat: implement dot repeat (.) command
- Add RepeatableChange enum with variants for OperatorMotion, OperatorTextObject, DeleteCharAt, InsertText
- Track last_change in EditorState for repeat functionality
- Track insert_buffer to capture text entered in insert mode
- Handle RepeatLastChange action to replay stored changes
- Fix EnterInsertModeEndOfLine to correctly position cursor past last character
- Add 3 tests for dot repeat

### feat: implement marks (m, `, ')
- Add marks storage (HashMap<char, LineCol>) to EditorState
- Add SetMark, JumpToMarkExact, JumpToMarkLine actions
- Add ToMarkExact, ToMarkLine motion variants
- Handle m{mark} to set mark, `{mark} to jump to exact position, '{mark} to jump to line
- Support both lowercase and uppercase marks (a-z, A-Z)
- Add 2 tests for mark set and jump functionality

### feat: implement substitute command (:s/pattern/replacement/flags)
- Add Substitute action with pattern, replacement, and flags
- Parse substitute command in CommandParser (supports / # | delimiters)
- Add apply_substitute method to EditorState
- Add replace_line method to Buffer for in-place line modification
- Support 'g' flag for global replacement on line
- Add 5 tests for substitute parsing and execution

### feat: implement named registers ("a, "b, etc.)
- Add registers HashMap<char, String> to EditorState for named register storage
- Add pending_register Option<char> for tracking selected register
- Add SetPendingRegister action to set the register for next yank/delete/paste
- Add "{register} handling in mode handler (f/t/m/`/' consolidated with register selection)
- Modify yank/delete operations to store in pending register when set
- Modify paste to use pending register content when set
- Fix yank motion to restore cursor position after yank (vim-compatible)
- Add 2 tests for named register yank and paste

### feat: implement macros (q{register}, @{register}, @@)
- Add macro_recording_register, macro_recording_keys, last_macro_register to EditorState
- Add ToggleMacroRecording, PlayMacro, RepeatLastMacro actions
- Add q{register} to start/stop recording, @{register} to playback, @@ to repeat last
- Intercept 'q' during recording to stop (handled in EditorState before mode handler)
- Implement Display trait for KeyInput to serialize keys as strings
- Prevent recursive recording during macro playback
- Add 2 tests for macro recording and playback

### feat: implement jump lists (Ctrl-o, Ctrl-i)
- Add jump_list and jump_list_index fields to EditorState
- Add JumpListOlder and JumpListNewer actions
- Add add_to_jump_list() for recording jump positions
- Add jump_list_older() and jump_list_newer() navigation
- Add jump list calls to: FileStart, FileEnd, search, mark jumps
- Add Ctrl-o and Ctrl-i key bindings in normal mode
- Handle branch behavior when jumping back then making new jump
- Limit jump list size to 100 entries (Vim-compatible)
- Add 2 tests for jump list functionality

### feat: implement block visual mode (Ctrl-v)
- Add EnterVisualBlockMode action
- Add Ctrl-v key binding in normal mode
- Add block selection handling in visual operators:
  - Block delete: delete rectangular region from all lines
  - Block yank: yank rectangular region
  - Block change: delete rectangular region and enter insert
- Preserve line numbers by deleting from bottom to top
- Add 2 tests for block visual mode

### feat: implement global command (:g and :v)
- Add Global action with pattern, command, and invert flag
- Add g/pattern/command and v/pattern/command parsing
- Implement apply_global() to execute commands on matching lines
- Support delete (d) as default command
- Support substitute (s/old/new/) on matching lines
- Handle :v (vglobal) for inverted matching
- Delete from bottom to top to preserve line numbers
- Add 2 tests for global and vglobal commands

### feat: implement changelist navigation (g;, g,)
- Add change_list and change_list_index fields to EditorState
- Add ChangeListOlder and ChangeListNewer actions
- Add add_to_change_list() for recording change positions
- Add change_list_older() and change_list_newer() navigation
- Track changes in: ReturnToNormalMode (insert), DeleteCharAt, DeleteLine, operator motions, operator text objects
- Add g; and g, key bindings in normal mode
- Limit change list size to 100 entries (Vim-compatible)
- Add 2 tests for changelist functionality

### feat: implement sentence and paragraph motions ((, ), {, })
- Add SentenceForward, SentenceBackward, ParagraphForward, ParagraphBackward motions
- Add corresponding actions to EditorAction enum
- Implement move_sentence_forward/backward (based on .!? punctuation)
- Implement move_paragraph_forward/backward (based on blank lines)
- Add motion support for operators (d(, c}, etc.)
- Add (, ), {, } key bindings in normal mode
- Add 4 tests for sentence/paragraph motions

### feat: implement match bracket motion (%)
- Add MatchBracket motion
- Implement move_match_bracket with bracket matching
- Support (), [], {} bracket pairs
- Implement find_matching_bracket_forward/backward helpers
- Scan forward for matching close, backward for matching open
- Handle nested brackets correctly
- Add motion support for operators (d%, c%, etc.)
- Add 3 tests for match bracket functionality

## Current Test Count

Total: 127 tests passing

| Crate | Count |
|-------|-------|
| kjxlkj | 2 |
| kjxlkj-core-types | 8 |
| kjxlkj-core-text | 7 |
| kjxlkj-core-undo | 3 |
| kjxlkj-core-edit | 18 |
| kjxlkj-core-mode | 6 |
| kjxlkj-core-state | 68 |
| kjxlkj-core-ui | 5 |
| kjxlkj-input | 2 |
| kjxlkj-render | 1 |
| kjxlkj-services | 1 |
| kjxlkj-service-fs | 5 |
| kjxlkj-service-terminal | 1 |

## Files Over 200 Lines

| File | Lines |
|------|-------|
| kjxlkj-core-state/src/editor.rs | ~3150 |
| kjxlkj-core-mode/src/handler.rs | ~790 |
| kjxlkj-core-edit/src/cursor_ops.rs | ~510 |
| kjxlkj-core-edit/src/text_objects.rs | ~460 |
| kjxlkj-core-edit/src/buffer.rs | ~420 |
| kjxlkj-core-types/src/event.rs | ~295 |
| kjxlkj-core-text/src/rope_text.rs | ~260 |
| kjxlkj-service-fs/src/service.rs | ~250 |
| kjxlkj-core-state/src/command.rs | ~210 |

## Features Implemented

### Editing
- [x] Operator+motion framework (d/y/c with motions)
- [x] Line operators (dd, yy, cc, >>, <<)
- [x] Text objects (iw, aw, i", a", i(, a(, etc.)
- [x] Search (/, ?, n, N)
- [x] Visual mode operators (d, y, c in v/V mode)
- [x] Find char motions (f, t, F, T)
- [x] Repeat find char (;, ,)
- [x] Dot repeat (.)
- [x] Marks (m, ', `)
- [x] Substitute command (:s)
- [x] Named registers ("a, "b, etc.)
- [x] Macros (q{register} to record, @{register} to play, @@ to repeat)
- [x] Jump lists (Ctrl-o, Ctrl-i)
- [x] Block visual mode (Ctrl-v)
- [x] Global command (:g, :v)
- [x] Changelist (g;, g,)
- [x] Sentence/paragraph motions ((, ), {, })
- [x] Match bracket motion (%)

### Pending
- [ ] Search highlighting
- [ ] Additional Ex commands (:set, :map, etc.)

## Next Steps

1. Add more Ex commands (:set, :map, etc.)
2. Implement search highlighting
3. Review TODO checklists for remaining items
