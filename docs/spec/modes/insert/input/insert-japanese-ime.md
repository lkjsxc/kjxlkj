# Japanese IME Behavior

Back: [/docs/spec/modes/insert/input/README.md](/docs/spec/modes/insert/input/README.md)

Production-grade Japanese IME behavior for terminal Insert mode.

## Scope

- hiragana/katakana/kanji composition
- commit/cancel safety under modal editing
- leader mapping isolation during composition
- width/wrap correctness after commit

## Composition State Model

| State | Description | Buffer Mutation |
|---|---|---|
| `Idle` | no active composition | direct insert path |
| `Preedit` | composing text | committed buffer unchanged |
| `CandidateSelect` | selecting conversion candidate | committed buffer unchanged |
| `Committed` | candidate accepted | atomic insert transaction |
| `Cancelled` | composition aborted | committed buffer unchanged |

## Routing Priority (normative)

While composing, IME routing executes before leader and normal key mappings.

| Key | Required Behavior During Composition |
|---|---|
| `Space` | candidate cycle/selection only; MUST NOT trigger `<leader>` |
| `Enter` | commit candidate |
| `Esc` | cancel composition first |
| `Backspace` | edit preedit text |

Only after returning to `Idle` may normal Insert key handling continue.

## Commit and Undo Rules

| Requirement | Detail |
|---|---|
| Atomic commit | committed string inserts as one logical edit |
| Undo grouping | one commit should undo as one unit |
| Cursor placement | cursor moves to end of committed grapheme sequence |
| UTF-8 validity | committed text remains valid UTF-8 |

## Width and Wrapping Rules

Committed text MUST satisfy:

- grapheme-safe cursor semantics from [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- wrap safety from [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

Additional requirements:

- width-2 graphemes MUST NOT create half-cell cursor states
- wrap boundaries MUST pad instead of splitting width-2 graphemes

## Failure Handling

| Failure | Required Behavior |
|---|---|
| IME backend unavailable | fallback to direct Unicode input with warning |
| candidate decode error | cancel composition; committed text unchanged |
| transport ambiguity | prefer safe cancel over accidental mode exit |

## Mandatory Verification

| ID | Scenario |
|---|---|
| `JP-01` | composition start, candidate cycle, commit |
| `JP-02` | cancel leaves committed text unchanged |
| `JP-03` | `Space` during composition does not trigger leader mappings |
| `JP-04` | long Japanese commit preserves wrap and cursor invariants |
| `JP-05` | post-commit `a` and `A` behavior remains correct |
| `JP-06R` | IME + `<leader>e` race does not trigger explorer action |
| `JP-07R` | IME + `<leader>t` race does not trigger terminal action |
| `JP-08R` | composition cancel followed by `Esc` exits Insert exactly once |
| `JP-09R` | mixed IME composition under resize keeps cursor visible |

## Related

- Input decoding: [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- Mode entry keybindings: [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- E2E testing: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
