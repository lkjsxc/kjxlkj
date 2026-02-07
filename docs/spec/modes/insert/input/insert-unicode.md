# Insert Unicode Input

Back: [/docs/spec/modes/insert/input/README.md](/docs/spec/modes/insert/input/README.md)

This document defines Unicode insertion behavior in Insert mode.

## Goals

| Goal | Requirement |
|---|---|
| Data integrity | UTF-8 text MUST be preserved without silent corruption. |
| Determinism | Identical input events MUST yield identical buffer text and cursor state. |
| Predictability | Width and grapheme behavior MUST be stable enough for cursor/viewport invariants. |

## Supported entry paths

| Path | Requirement |
|---|---|
| Direct character input | Printable Unicode committed by terminal/IME MUST insert at the current insertion point. |
| Literal insertion (`Ctrl-v` forms) | Numeric or escaped literal forms MUST insert the intended Unicode scalar value. |
| Digraph insertion (`Ctrl-k`) | Registered digraphs MUST map to deterministic Unicode output. |
| Register insertion (`Ctrl-r`) | Register text MUST insert exactly as stored UTF-8 bytes. |

## Cursor and grapheme semantics

| Topic | Requirement |
|---|---|
| Insert cursor model | Insert mode uses end-inclusive insertion point (`0..N`). |
| Grapheme atomicity | Delete/backspace operations SHOULD treat one grapheme cluster as one visible unit. |
| Normal-mode clamp | Exiting Insert MUST clamp cursor back to end-exclusive model. |

## Width and display interaction

| Character class | Expected display behavior |
|---|---|
| ASCII and most Latin | width 1 |
| Many CJK characters | width 2 |
| Combining marks and joiners | width 0 by themselves; modify previous grapheme |

Width differences MUST NOT break cursor visibility or viewport clamping.

## Composition safety

Insert operations MUST separate two phases:

| Phase | Requirement |
|---|---|
| Preedit/composition | In-progress composition text MUST NOT be committed as buffer text yet. |
| Commit | Final committed text MUST be inserted atomically at the insertion point. |

`Esc` during composition MUST cancel preedit first. Mode transition out of Insert occurs only after composition is cancelled or committed.

## Normalization policy

The editor MUST NOT silently normalize buffer text between NFC/NFD during ordinary typing.

If search or compare paths use normalization, the behavior MUST be documented and test-covered separately.

## Required regression tests

| Category | Required cases |
|---|---|
| Unit/integration | wide character insertion, combining mark insertion, literal insertion, digraph insertion |
| Boundary | insertion at start/end of line, empty line, and long wrapped lines |
| Mode transition | repeated Insert enter/exit with Unicode text never leaves floating Normal-mode cursor |
| PTY E2E | interactive Unicode typing path writes expected UTF-8 bytes to disk via `:wq` |

## Related

- Insert mode: [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- Cursor contract: [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- Unicode implementation guidance: [/docs/technical/unicode.md](/docs/technical/unicode.md)
