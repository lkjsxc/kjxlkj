# Large File Report

Back: [/docs/log/README.md](/docs/log/README.md)

## Files Exceeding 200 Lines

Per the constraint that files exceeding 200 lines should be noted in documentation, the following source files exceed this threshold:

| File | Lines | Notes |
|------|-------|-------|
| `src/crates/kjxlkj-core-state/src/editor.rs` | 768 | Core editor state machine - contains all editor logic |
| `src/crates/kjxlkj-core-edit/src/motion.rs` | 476 | Motion implementations - many motion types |
| `src/crates/kjxlkj-core-mode/src/parser.rs` | 473 | Key parser - comprehensive key handling |
| `src/crates/kjxlkj-core-types/src/event.rs` | 353 | Event types - many intent/motion variants |
| `src/crates/kjxlkj-core-edit/src/text_object.rs` | 338 | Text object implementations |
| `src/crates/kjxlkj-core-text/src/text_buffer.rs` | 295 | Text buffer with rope operations |
| `src/crates/kjxlkj-core-edit/src/operator.rs` | 267 | Operator implementations |
| `src/crates/kjxlkj-core-undo/src/history.rs` | 258 | Undo history implementation |
| `src/crates/kjxlkj-core-ui/src/snapshot.rs` | 244 | UI snapshot structures |
| `src/crates/kjxlkj-render/src/renderer.rs` | 235 | Terminal rendering |

## Rationale

These files are intentionally larger because:

1. **Cohesion**: Splitting these files would fragment closely related logic across multiple modules
2. **Editor complexity**: A modal editor requires substantial logic for mode handling, motions, operators
3. **Type definitions**: Enums with many variants (events, intents) naturally require more lines
4. **Tests included**: Several files include inline unit tests

## Future Improvements

- Consider splitting `editor.rs` into separate modules for:
  - Buffer management
  - Window management
  - Command execution
  - Search functionality
- Consider extracting motion implementations by category (basic, word, paragraph)
- Consider moving type definitions to separate files per category
