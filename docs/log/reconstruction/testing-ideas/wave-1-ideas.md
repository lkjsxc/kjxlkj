# Implementation Wave 1 - Ideas

## Cursor Position in Snapshot

The EditorSnapshot should include cursor position information for proper rendering.
Currently the cursor is placed at (0,0) by the painter.

### Proposed Changes

1. Add `cursor` field to `TabSnapshot` or a separate `WindowSnapshot` type
2. Include cursor line and column in the snapshot
3. Use this in the painter to position the terminal cursor correctly

## Command Line Editing

The command mode currently stores cmdline content but doesn't update it on keystrokes.
Need to wire up the command mode handler to actually edit the cmdline buffer.

## File Write Implementation

The `:w` command is stubbed. Need to:
1. Add file write to kjxlkj-service-fs
2. Wire EditorState to use the fs service for writes
3. Update buffer.meta.modified after successful write

## Viewport Scrolling

The snapshot returns all buffer lines. For large files:
1. EditorSnapshot should only include visible lines
2. Need viewport tracking per window
3. Scroll actions should update viewport offsets

## Operator-Pending Mode

The dispatch for operator-pending mode needs:
1. Second key dispatch for motion/text-object
2. Region calculation
3. Actual operator execution (delete, yank, change)

## Testing Ideas

1. Add integration test that spawns the app in headless mode
2. Test key sequence -> state change -> snapshot output
3. Property-based testing for motion calculations
