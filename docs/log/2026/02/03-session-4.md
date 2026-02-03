# 2026-02-03: Session 4 - More Features

Back: [/docs/log/2026/02/README.md](/docs/log/2026/02/README.md)

## Summary

Continued implementation: join lines (J/gJ), case operators, increment/decrement, star search, insert mode commands, Replace mode, and single character replace.

## Commits This Session

### feat: implement join lines (J, gJ)
- 129 tests

### feat: implement case operators (~, g~, gU, gu)
- Added ToggleCaseChar, ToggleCaseLine, ToggleCaseMotion actions
- Added UppercaseLine, UppercaseMotion, LowercaseLine, LowercaseMotion actions
- Added CaseTransform enum (Toggle, Upper, Lower)
- 133 tests

### feat: implement increment/decrement numbers (Ctrl-A, Ctrl-X)
- Finds number at/after cursor, modifies it
- Handles negative numbers
- 136 tests

### feat: implement star search (* and #)
- Search for word under cursor
- 138 tests

### feat: implement open line above (O)
- 139 tests

### feat: implement delete before (X) and paste before (P)
- X key binding for DeleteCharBefore
- PasteBefore action for P command
- 140 tests

### feat: implement insert mode commands (Ctrl-w, Ctrl-u, Ctrl-r)
- Ctrl-w: delete word before cursor
- Ctrl-u: delete to line start
- Ctrl-r {reg}: insert register contents
- 143 tests

### feat: implement proper Replace mode (R)
- ReplaceChar action for overwriting characters
- Backspace moves cursor left
- At end of line, characters are inserted
- 144 tests

### feat: implement single character replace (r)
- ReplaceSingleChar action for r{char} command
- Does not advance cursor
- 145 tests

## Current Test Count: 145

## Files Over 200 Lines

| File | Lines |
|------|-------|
| kjxlkj-core-state/src/editor.rs | ~4168 |
| kjxlkj-core-mode/src/handler.rs | ~955 |
| kjxlkj-core-edit/src/buffer.rs | ~565 |
| kjxlkj-core-edit/src/cursor_ops.rs | ~510 |
| kjxlkj-core-edit/src/text_objects.rs | ~460 |
| kjxlkj-core-types/src/event.rs | ~334 |
| kjxlkj-core-text/src/rope_text.rs | ~260 |
| kjxlkj-service-fs/src/service.rs | ~250 |
| kjxlkj-core-state/src/command.rs | ~210 |

## Features Completed

- [x] Join lines (J, gJ)
- [x] Case operators (~, g~, gU, gu)
- [x] Increment/decrement (Ctrl-A, Ctrl-X)
- [x] Star search (*, #)
- [x] Open line above (O)
- [x] Delete before (X)
- [x] Paste before (P)
- [x] Insert mode commands (Ctrl-w, Ctrl-u, Ctrl-r)
- [x] Replace mode (R)
- [x] Single character replace (r)

## Next Steps

1. Continue checking TODO checklists for remaining items
2. Focus on any remaining core editing features
3. Update documentation and conformance
