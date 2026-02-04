# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)
Current implementation surface relative to the canonical spec.

## Purpose

The canonical spec under `/docs/spec/` describes the target system.

This document set records the currently implemented, user-visible surface so that:

- spec language is not misread as “already implemented”
- tests can map to explicit supported behavior
- gaps are explicit and actionable

## Conformance documents (current surface)

| Document | Content |
|----------|---------|
| [CONFORMANCE_MODES_KEYS.md](CONFORMANCE_MODES_KEYS.md) | Modes and keybindings currently supported |
| [CONFORMANCE_EDITING.md](CONFORMANCE_EDITING.md) | Editing semantics (operators, text objects, search behavior) |
| [CONFORMANCE_COMMANDS_TESTING.md](CONFORMANCE_COMMANDS_TESTING.md) | Ex commands and headless/E2E harness surface |

## How to use this (recommended)

1. Read the target spec for the area you care about (under `/docs/spec/`).
2. Check this conformance set to confirm what is implemented today.
3. If there is drift:
   - update docs and/or implementation, and
   - record user-visible gaps in `/docs/reference/LIMITATIONS.md`.

## Related

- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Spec index: [/docs/spec/README.md](/docs/spec/README.md)
