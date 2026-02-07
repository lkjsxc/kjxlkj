# Japanese IME Behavior

Back: [/docs/spec/modes/insert/input/README.md](/docs/spec/modes/insert/input/README.md)

This document specifies Japanese input behavior in Insert mode.

## Scope

Covers Hiragana, Katakana, Kanji conversion, and mixed Japanese/ASCII editing through terminal IME pipelines.

## Composition model

| Stage | Requirement |
|---|---|
| Preedit start | IME preedit text is transient UI state, not committed buffer text. |
| Candidate conversion | Candidate cycling MUST not mutate buffer text until commit. |
| Commit | Confirmed candidate string is inserted atomically at the insertion point. |
| Cancel | Cancelling conversion discards transient preedit text and keeps committed buffer unchanged. |

## Key behavior requirements

| Key | Requirement during Japanese composition |
|---|---|
| `Space` | Used for conversion/candidate cycling while composition is active. |
| `Enter` | Commits current conversion candidate. |
| `Esc` | Cancels composition first; only leaves Insert mode when no active composition remains. |
| `Backspace` | Deletes within composition text before commit; after commit it acts on committed grapheme text. |

## Leader and command safety

Japanese composition MUST NOT leak intermediate key events into Normal-mode mappings.

In particular:

- `Space` used inside IME conversion MUST NOT trigger leader mappings.
- Partial conversion sequences MUST NOT open command line or switch mode unexpectedly.

## Width and wrapping

| Topic | Requirement |
|---|---|
| Full-width glyphs | Cursor and viewport calculations MUST account for width-2 glyphs. |
| Wrapped lines | Long Japanese lines that overflow width MUST wrap to next display row when `wrap = true`. |
| Mixed scripts | Mixed Latin/Japanese text MUST keep deterministic cursor column mapping. |

## Required tests

| Category | Required scenarios |
|---|---|
| PTY E2E | type Japanese text via IME path, commit, `:wq`, and verify UTF-8 file bytes |
| PTY E2E | hold repeated `a`, type Japanese text, `Esc`, verify cursor clamps correctly |
| Integration | cancellation path (`Esc`) during active preedit leaves buffer unchanged |
| Integration | `Space` during conversion does not trigger leader mappings |

If a platform cannot run IME PTY automation reliably, record the gap in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) and keep manual reproduction steps in `/docs/log/`.

## Related

- Unicode input: [/docs/spec/modes/insert/input/insert-unicode.md](/docs/spec/modes/insert/input/insert-unicode.md)
- Keybinding contract: [/docs/spec/ux/keybindings.md](/docs/spec/ux/keybindings.md)
- Viewport wrapping: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
