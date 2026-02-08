# Mark Types

Back: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)

Classification of all mark types and their storage scope.

## Local marks (lowercase)

| Range | Scope | Description |
|---|---|---|
| `a` - `z` | Buffer-local | User-set marks, one per letter per buffer |

Local marks are valid only within the buffer where they were set. They persist across sessions if session persistence is enabled.

## Global marks (uppercase)

| Range | Scope | Description |
|---|---|---|
| `A` - `Z` | Global | User-set marks, one per letter across all buffers |

Global marks remember both the file path and the position. Jumping to a global mark opens the associated file.

## Numbered marks

| Range | Scope | Description |
|---|---|---|
| `0` - `9` | Global | Automatically set by the editor |

`0` stores the position where the editor last exited. `1`-`9` store the previous exit positions (shifted like numbered registers).

## Special marks

| Mark | Description |
|---|---|
| `` ` `` | Position before the last jump |
| `'` | Line of position before the last jump |
| `.` | Position of the last change |
| `"` | Position when last exiting the buffer |
| `^` | Position where Insert mode was last stopped |
| `[` | Start of the last changed or yanked text |
| `]` | End of the last changed or yanked text |
| `<` | Start of the last visual selection |
| `>` | End of the last visual selection |

## Automatic marks

Automatic marks are set by the editor without user action. They cannot be set explicitly by the user.

## Mark validity

A mark becomes invalid when:

- The line it points to is deleted
- The buffer is unloaded (for local marks)
- The file is deleted (for global marks)

Invalid marks are silently removed. Jumping to an invalid mark produces an error.

## Storage

| Mark type | Stored in |
|---|---|
| Local marks | Buffer state (core-owned) |
| Global marks | Editor state (core-owned) |
| Special marks | Automatically maintained by core |

## Related

- Automatic marks: [/docs/spec/editing/marks/automatic-marks.md](/docs/spec/editing/marks/automatic-marks.md)
- Special marks: [/docs/spec/editing/marks/special-marks.md](/docs/spec/editing/marks/special-marks.md)
- Jump marks: [/docs/spec/editing/marks/jump-marks.md](/docs/spec/editing/marks/jump-marks.md)
- Mark persistence: [/docs/spec/editing/marks/mark-persistence.md](/docs/spec/editing/marks/mark-persistence.md)
