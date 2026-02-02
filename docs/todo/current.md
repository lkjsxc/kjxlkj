# Current TODO

Active task list for implementation - Iteration 27.

## Active Tasks

| ID | Task | Status |
|----|------|--------|
| 1 | Iteration 27 complete | ✓ complete |

## Completed (Iteration 27)

- [x] Implement ex command execution module
- [x] Handle :w, :q, :wq, :e commands
- [x] Handle :sp, :vsp, :close, :only commands
- [x] Handle :bn, :bp buffer navigation
- [x] Add OpenFile action result
- [x] Add active_buffer_mut() to EditorState
- [x] All 833 tests passing

## Implementation Summary (Iteration 27)

### New Module Added

| Module | Lines | Purpose |
|--------|-------|---------|
| ex_command.rs | 150 | Ex command parsing and execution |

### Ex Commands Supported

| Command | Description |
|---------|-------------|
| :w, :write | Save buffer |
| :q, :quit | Quit editor |
| :wq, :x | Save and quit |
| :e, :edit | Open file |
| :sp, :split | Horizontal split |
| :vsp, :vsplit | Vertical split |
| :close | Close window |
| :only | Keep only current window |
| :bn, :bnext | Next buffer |
| :bp, :bprev | Previous buffer |

## Related

- Plan: [plan.md](plan.md)
- Completed: [completed.md](completed.md)
