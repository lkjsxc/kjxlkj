# Japanese IME Behavior

Back: [/docs/spec/modes/insert/input/README.md](/docs/spec/modes/insert/input/README.md)

This specification defines production-grade Japanese IME behavior in terminal
Insert mode.

## Scope

- Hiragana, katakana, and kanji composition in terminal environments
- commit/cancel safety under modal editing
- leader mapping isolation during composition
- CJK width and wrap correctness after commit

## Composition State Model

| State | Description | Buffer Mutation |
|---|---|---|
| `Idle` | no active composition | direct insert path |
| `Preedit` | composing candidate text | committed buffer unchanged |
| `CandidateSelect` | candidate list navigation | committed buffer unchanged |
| `Committed` | candidate accepted | atomic insert transaction |
| `Cancelled` | composition aborted | committed buffer unchanged |

## Input Routing Rules

| Rule | Requirement |
|---|---|
| Composition priority | While composing, IME handler consumes keys before Normal/leader mapping logic |
| Space safety | `Space` for candidate cycling MUST NOT trigger `<leader>` actions |
| Escape priority | `Esc` cancels composition first; only a subsequent `Esc` may leave Insert |
| Enter behavior | `Enter` commits candidate during composition; otherwise Insert newline |
| Backspace behavior | During composition edits preedit text; otherwise edits committed text |

## Commit and Undo Rules

| Requirement | Detail |
|---|---|
| Atomic commit | committed string inserts as one logical operation |
| Undo grouping | one composition commit SHOULD be undone as one unit |
| Cursor placement | insertion point moves to end of committed grapheme sequence |
| UTF-8 validity | committed text remains valid UTF-8 |

## Width and Wrapping Rules

Committed Japanese text MUST satisfy:

- grapheme-safe cursor semantics from
  [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- on-screen wrapping from
  [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

Additional rules:

- width-2 graphemes MUST NOT create half-cell cursor states
- wrap boundaries MUST pad instead of splitting width-2 graphemes

## Failure Handling

| Failure | Required Behavior |
|---|---|
| IME backend unavailable | fallback to direct Unicode input with warning notification |
| Candidate decode error | cancel composition and keep committed text unchanged |
| transport ambiguity | prefer safe composition cancel over accidental mode exit |

## Mandatory Verification

| ID | Scenario |
|---|---|
| JP-01 | composition start, candidate cycle, commit |
| JP-02 | composition cancel leaves committed text unchanged |
| JP-03 | composition `Space` does not trigger leader mapping |
| JP-04 | long Japanese commit preserves wrap and cursor invariants |
| JP-05 | post-commit `a` and `A` behavior remains correct |

## Related

- Insert mode: [/docs/spec/modes/insert/insert.md](/docs/spec/modes/insert/insert.md)
- Keybindings: [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- E2E tests: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
