# Japanese IME Behavior

Back: [/docs/spec/modes/insert/input/README.md](/docs/spec/modes/insert/input/README.md)

This spec defines production-grade Japanese IME behavior for Insert mode.

## Scope

- Hiragana/Katakana/Kanji composition in terminal environments
- Commit/cancel semantics with modal safety
- Interaction with leader mappings and command mode entry
- Width-aware cursor and wrap behavior for committed text

## Composition State Machine (Normative)

| State | Description | Buffer Mutation |
|---|---|---|
| `Idle` | No active composition | Direct insert for non-IME keys |
| `Preedit` | IME candidate string in progress | MUST NOT mutate committed buffer text |
| `CandidateSelect` | Candidate list/cycle active | MUST NOT mutate committed buffer text |
| `Committed` | Candidate accepted | MUST atomically insert committed string |
| `Cancelled` | Composition aborted | MUST leave committed buffer unchanged |

## Input Routing Rules

| Rule | Requirement |
|---|---|
| Composition interception | While in `Preedit` or `CandidateSelect`, key events MUST be consumed by IME handler before Normal/leader mappings. |
| Leader safety | `Space` used for candidate cycling MUST NOT trigger leader mappings. |
| Escape priority | `Esc` MUST cancel composition first; only after composition is idle may it exit Insert mode. |
| Enter behavior | `Enter` during composition MUST commit candidate; outside composition it inserts newline per Insert rules. |
| Backspace behavior | During composition, `Backspace` edits preedit text; outside composition it edits committed buffer text. |

## Commit Semantics

| Requirement | Detail |
|---|---|
| Atomic commit | Committed string (possibly multi-grapheme) MUST be inserted as one logical insert transaction. |
| Undo behavior | A single composition commit SHOULD be undoable as one unit. |
| Cursor placement | After commit, cursor insertion point MUST move to the end of committed grapheme sequence. |
| UTF-8 integrity | Committed text MUST remain valid UTF-8 and preserve grapheme boundaries. |

## Cancel Semantics

| Requirement | Detail |
|---|---|
| No side effect | Cancelling composition MUST NOT alter committed buffer text. |
| Mode continuity | Cancel MUST keep editor in Insert mode unless a second explicit mode-exit key is issued. |
| UI cleanup | Preedit/candidate UI state MUST be cleared immediately on cancel. |

## Width and Wrapping

Committed Japanese text MUST follow cursor and viewport rules in:

- [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md)
- [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

Additional requirements:

- Wide graphemes MUST never produce half-cell cursor states.
- Wrap boundary logic MUST pad when a width-2 grapheme would split rows.

## Failure Handling

| Failure | Required Behavior |
|---|---|
| IME backend unavailable | Fall back to direct Unicode input with warning notification |
| Candidate decode error | Abort composition safely; keep committed text unchanged |
| Terminal transport ambiguity | Prefer composition cancellation over accidental mode transitions |

## Mandatory Test Coverage

| ID | Scenario |
|---|---|
| JP-01 | Start composition, cycle candidates with `Space`, commit with `Enter` |
| JP-02 | Start composition, cancel with `Esc`, verify no buffer mutation |
| JP-03 | Composition active while leader key is configured; ensure no leader command fires |
| JP-04 | Commit long Japanese phrase and verify wrap/cursor invariants |
| JP-05 | Repeat `A`/append workflows after Japanese commit and ensure cursor clamp correctness |

Detailed scenario drafts: [/docs/log/reconstruction/testing-ideas/2026-02-09-e2e-boundary-blueprint.md](/docs/log/reconstruction/testing-ideas/2026-02-09-e2e-boundary-blueprint.md)

## Related

- Insert mode: [/docs/spec/modes/insert/insert.md](/docs/spec/modes/insert/insert.md)
- Keybinding behavior: [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)
- Known current gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
