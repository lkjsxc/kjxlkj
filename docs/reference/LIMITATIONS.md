# Known Limitations

Back: [/docs/reference/README.md](/docs/reference/README.md)

User-visible gaps and caveats relative to the target spec.

## Purpose

The target behavior is defined in `/docs/spec/`. This document records user-visible drift and known rough edges in the current implementation.

## Status sources

Do not infer "implemented" from target specs or placeholder feature lists. Authoritative sources for "what exists" are:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) (the supported surface)
- The repository's automated tests (when an implementation workspace exists)

## Entry discipline (normative)

Each user-visible limitation entry MUST include:

| Field | Requirement |
|---|---|
| Expected behavior | Exact `/docs/spec/...` link defining the target behavior. |
| Observed behavior | What the user actually sees. |
| Status | One of `open`, `partial`, or `accepted-temporary`. |
| Evidence | Test path, failing repro command, or deterministic manual script. |
| Next action | Fix path or explicit deferral plan. |

Entries without evidence are allowed only as temporary triage notes.

## Relationship to conformance (normative)

- `CONFORMANCE` declares supported behavior.
- `LIMITATIONS` records exceptions and gaps.

If `CONFORMANCE` says `implemented` but a user-visible defect exists, change conformance to `partial` until the limitation is closed.

## High-priority UX regression watchlist

These scenarios are high risk because they can regress without being caught by headless-only checks.

| Scenario | Expected behavior | Defining spec | Required evidence |
|---|---|---|---|
| Leader key reachability | `Space` acts as `<leader>` in Normal; `<leader>e` and `<leader>t` reachable | [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md) | PTY E2E |
| Append at EOL (`a`) | `a` on last char enters Insert at true EOL; `Esc` returns without floating cursor | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | PTY E2E |
| Soft wrap behavior | Long lines wrap when `wrap = true` (default) | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | PTY E2E |
| CJK cursor correctness | Cursor never occupies half-cell of wide character | [/docs/spec/editing/cursor/README.md](/docs/spec/editing/cursor/README.md) | PTY E2E |
| CJK wrap boundary | Width-2 char at row end produces padding, not split | [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md) | PTY E2E |
| Terminal as window | Terminal is a window navigable with `Ctrl-w` | [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md) | PTY E2E |
| Session JSON roundtrip | `:SessionSave` then `:SessionLoad` restores layout | [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md) | Integration |
| C language detection | Built-in detection includes C/C++ by extension | [/docs/spec/features/syntax/syntax-files.md](/docs/spec/features/syntax/syntax-files.md) | Integration |

## Current implementation status

No implementation currently exists in the repository. The repository is in a docs-only state, awaiting reconstruction from the canonical spec.

All features defined in `/docs/spec/` are `planned` until a reconstruction wave produces verifiable code.

## Update protocol

When the implementation changes:

1. Update the relevant conformance entry.
2. Update this document for user-visible gaps.
3. Run verification from [/docs/reference/CI.md](/docs/reference/CI.md).
4. Keep conformance and test reality synchronized.

## Related

- Conformance ledger: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Spec index: [/docs/spec/README.md](/docs/spec/README.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Anti-MVP measures: [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md)
