# Gap Closure Plan

Closing the 6 remaining gaps from LIMITATIONS.md.

## Gaps to Close

| # | Gap ID | Req ID | Description | Priority |
|---|--------|--------|-------------|----------|
| 1 | LIM-GAP-REG-01 | R-EDIT-07 | Named registers | high |
| 2 | LIM-GAP-REPLACE-01 | R-MODE-05 | Replace mode overwrite | high |
| 3 | LIM-GAP-VISUAL-01 | R-MODE-03 | Visual mode selection + operators | high |
| 4 | LIM-GAP-EXP-02 | R-EXP-02, R-EXP-04 | Explorer nav + file ops | medium |
| 5 | LIM-GAP-SESS-01 | R-SESS-03 | Auto-session on exit/startup | medium |
| 6 | LIM-GAP-PTY-01 | R-TERM-03 | Terminal resize propagation | medium |
| 7 | — | R-CMD-03 | :w path file write | medium |

## Implementation Order

1. Named registers (foundational — visual/replace use registers)
2. Replace mode overwrite
3. Visual mode selection + operators
4. Explorer navigation + file ops
5. :w path file write
6. Terminal resize propagation
7. Auto-session

## Progress

- [x] Gap 1: Named registers
- [x] Gap 2: Replace mode overwrite
- [x] Gap 3: Visual mode selection
- [x] Gap 4: Explorer nav + file ops
- [x] Gap 5: :w path file write
- [x] Gap 6: Terminal resize
- [x] Gap 7: Auto-session
- [x] All tests pass
- [x] All docs updated
