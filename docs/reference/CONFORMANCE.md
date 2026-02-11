# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports the strongest verified state as of the snapshot date.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is known |
| `partial` | behavior exists but reachability or evidence quality is incomplete |
| `blocked` | high-severity mismatch is known and not yet closed |
| `unverified` | no trustworthy evidence currently exists |

## Current Snapshot (2026-02-11)

Workspace reconstructed with 20 crates. Runtime conformance is partially verified
through 208 deterministic unit and integration tests covering key normalization,
mode dispatch, cursor motion, text buffer operations, layout tree, editor state,
multi-key sequences, operator composition, motion execution, motion type
classification, case operators, g-prefix operator dispatch, register system,
force modifiers, count multiplication, Vim regex compilation, ex command parsing,
search forward/backward with wrapping, command-line input handling, blackhole
register suppression, clipboard register stubs, \c/\C case sensitivity flags,
\o/\O octal and \H non-head atoms, put (p/P) paste operations, register-wired
yank/delete operators, and cursor boundary clamping.
Multi-task runtime architecture implemented (input/core/render tasks with bounded
channels, signal handlers, proper shutdown).
All source files comply with ≤ 200 line limit.
Motion system expanded to ~40 variants including find/till/paragraph/match-paren.
Operator enum expanded to 11 variants (Delete, Change, Yank, Indent, Dedent,
Reindent, Format, Lowercase, Uppercase, ToggleCase, Filter).
Operator composition implemented (linewise dd/yy/cc/guu/gUU/g~~/gqq,
operator+motion d3w/cw, D/Y/gJ special forms).
RangeType/Inclusivity classification system for motions implemented and tested.
Case transform operators (gu/gU/g~) on lines and ranges implemented.
PendingState system for multi-key normal mode sequences (count, g/z/f/t/r/m).
RegisterStore with named (a-z), numbered (0-9), unnamed, and small-delete
registers; yank records to unnamed+0, delete rotates 1-9 for linewise or writes
small-delete for non-linewise; A-Z append supported. Blackhole register ("_)
suppresses all register writes. Clipboard registers ("+, "*) store locally
(real clipboard integration deferred).
ForceModifier enum (Characterwise, Linewise, Blockwise) for v/V/Ctrl-v between
operator and motion in operator-pending mode.
Pre-operator count multiplication (e.g. 2d3w → count 6).
Dot-repeat recording via last_change tracking in EditorState.
Vim regex compiler translating magic-mode patterns to Rust regex (shortcut atoms,
word boundaries, grouping, alternation, quantifiers, \v very-magic switch, \c/\C
case sensitivity flags, \o/\O octal atoms, \H non-head atom, \= synonym for \?).
Ex command parser with abbreviation-based dispatch and ! force flag support.
Search system with forward/backward wrapping and compiled Vim regex patterns.
Command-line input handling for :, /, ? prefixes with mode transitions.
Put operations (p/P) paste from effective register with linewise/characterwise
handling. Operators wired to RegisterStore for yank/delete recording. Cursor
boundary clamping for post-edit safety.
Read-only registers: `"."` (last insert text), `"%"` (current filename),
`":"` (last ex command), `"/"` (last search pattern).
Insert-text session tracking for dot register. `:registers`/`:reg`/`:display`
command parsed. Last-command and last-search register wiring.
Star search (`*`) and hash search (`#`) for word under cursor with word-boundary
matching. `:nohlsearch`/`:noh` clears search highlighting. `hlsearch` state
tracks whether matches should be highlighted; new search re-enables highlight.
`match_count()` for total match reporting. `word_at()` word-under-cursor
extraction with comprehensive boundary tests. Search integration tests cover
multiline wrapping, empty buffer, non-word cursor, and search register wiring.
g*/g# partial match star search (no word boundaries). Search history tracking
with deduplication (capped at 100). ignorecase/smartcase settings with in-pattern
\c/\C override. Bracket matching (%) scans forward on current line when cursor
is not on a bracket character. bracket_pair() helper for bracket-type lookup.
Ctrl-a/Ctrl-x increment/decrement numbers under/after cursor with forward scan,
negative number and multi-digit support. :set/:se/:setlocal command for
ignorecase/smartcase/hlsearch options with no-prefix and key=value parsing.
Text objects (iw/aw/iW/aW, i(/a(/i{/a{/i[/a[/i</a</i>/a>, i"/a"/i'/a'/i`/a`)
with word, bracket (nesting-aware, multiline), and quote range computation.
Operator-pending text object dispatch via 'i'/'a' prefix keys.
Paragraph text objects (ip/ap) with contiguous non-blank line detection and
trailing blank inclusion for around variant. Sentence text objects (is/as) with
period/exclamation/question boundary detection and trailing whitespace inclusion.
Tree-sitter text objects (ic/ac, if/af) and tag objects (it/at) deferred.
Visual mode (v charwise, V linewise, Ctrl-v blockwise stub) with anchor/cursor
selection model, sub-mode switching, operator dispatch (d/x/y/c/s/>/</~/u/U/J/p),
anchor swap (o), and Escape exit. Blockwise visual operations delegate to
charwise (full block ops deferred).
PTY-level E2E verification pending harness reconstruction.

## Evidence Summary

| Check | Status | Evidence Date | Evidence |
|---|---|---|---|
| Docs authority and precedence are defined | `verified` | 2026-02-11 | [/docs/README.md](/docs/README.md), [/docs/policy/README.md](/docs/policy/README.md) |
| TODO reconstruction chain is present | `verified` | 2026-02-11 | [/docs/todo/README.md](/docs/todo/README.md), [/docs/todo/waves/README.md](/docs/todo/waves/README.md) |
| Implementation workspace is present | `verified` | 2026-02-11 | 20-crate workspace, `cargo check --workspace` and `cargo test --workspace` (208 pass) |
| Runtime blocker behavior (`Shift+a`, split, explorer) | `partial` | 2026-02-11 | T1 headless harness tests pass; T2 PTY harness pending |
| Live E2E screen-oracle closure | `unverified` | 2026-02-11 | PTY harness not yet reconstructed |

## Domain Status

| Domain | Status | Reason |
|---|---|---|
| Input decoding and key normalization | `partial` | Shift+a normalization implemented and unit-tested; T2 pending |
| Window tree and split lifecycle | `partial` | layout tree with split/close/rebalance implemented and unit-tested; T2 pending |
| Explorer window and actions | `unverified` | stub crate only; explorer state model not yet implemented |
| Terminal window integration | `unverified` | stub crate only; PTY not yet implemented |
| Viewport wrap and cursor safety | `unverified` | basic cursor motion; wrap not yet implemented |
| Test harness fidelity | `partial` | T1 headless harness with step dumps; T2 PTY harness pending |
| Source topology and workspace policy | `verified` | 20-crate grouped tree matches spec; all files ≤ 200 lines; multi-task runtime |

## Release Rule

Release conformance is not met while any high-severity limitation is open.

A release may proceed only when all are true:

1. all high-severity rows in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) are closed
2. matching `*R` E2E rows in [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md) pass using screen-state assertions
3. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md),
   [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md), and
   [/docs/todo/README.md](/docs/todo/README.md) are synchronized in the same change

Current state (2026-02-11): blocked (docs-only baseline).

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift rows: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Reconstruction checklist: [/docs/todo/README.md](/docs/todo/README.md)
