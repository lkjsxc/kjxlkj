# Current TODO

Active task list for implementation - Iteration 26.

## Active Tasks

| ID | Task | Status |
|----|------|--------|
| 1 | Implement operator execution | pending |
| 2 | Implement window operations | pending |
| 3 | Implement register operations | pending |
| 4 | Implement ex command execution | pending |
| 5 | Implement search operations | pending |
| 6 | Add integration tests | pending |

## Analysis (Iteration 26)

The following IntentKind variants need implementation:

### Operators (priority: high)
- `OperatorMotion` - apply operator over motion range
- `OperatorTextObject` - apply operator over text object range
- `OperatorLine` - apply operator on current line (dd, yy, etc)

### Window Operations (priority: medium)
- `SplitHorizontal`, `SplitVertical`
- `CloseWindow`, `OnlyWindow`
- `NextWindow`, `PrevWindow`, `WindowDirection`

### Register Operations (priority: medium)
- `PutAfter`, `PutBefore`
- `YankLine`

### Search Operations (priority: medium)
- `SearchForward`, `SearchBackward`
- `NextMatch`, `PrevMatch`

### Other
- `ExCommand` - execute parsed ex commands
- `Save`, `SaveQuit` - file operations
- `Repeat` - repeat last change

## Related

- Plan: [plan.md](plan.md)
- Completed: [completed.md](completed.md)
