# Special Marks

Back: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)

Automatically set marks for common positions.

## Overview

Special marks are set automatically by the editor. They cannot be set manually (with some exceptions).

## Special Mark List

| Mark | Position | Description |
|---|---|---|
| `` ` `` (backtick) | Before last jump | Position before the last jump command |
| `'` (single quote) | Before last jump (line) | Line of the last jump |
| `.` | Last change | Position of the last text change |
| `^` | Last insert | Position where insert mode was last exited |
| `[` | Start of last change | First character of last changed/yanked text |
| `]` | End of last change | Last character of last changed/yanked text |
| `<` | Start of visual | Start of last visual selection |
| `>` | End of visual | End of last visual selection |
| `"` | Last exit | Position when the buffer was last exited |
| `(` | Start of sentence | Start of current sentence |
| `)` | End of sentence | End of current sentence |
| `{` | Start of paragraph | Start of current paragraph |
| `}` | End of paragraph | End of current paragraph |

## Usage

| Command | Effect |
|---|---|
| `` `. `` | Jump to last change |
| `` `^ `` | Jump to last insert position |
| `` `" `` | Jump to position at last buffer exit |
| `` `[ `` | Jump to start of last change |
| `'<` | Jump to start line of last visual selection |

## Session Persistence

Marks `.`, `^`, `"`, `[`, `]`, `<`, `>` are saved per buffer in the session file.

## Related

- Marks overview: [/docs/spec/editing/marks/README.md](/docs/spec/editing/marks/README.md)
- Mark types: [/docs/spec/editing/marks/mark-types.md](/docs/spec/editing/marks/mark-types.md)
- Automatic marks: [/docs/spec/editing/marks/automatic-marks.md](/docs/spec/editing/marks/automatic-marks.md)
