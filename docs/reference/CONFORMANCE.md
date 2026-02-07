# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)
Conformance ledger for the implementation surface relative to the canonical spec.

## Purpose

The canonical spec under `/docs/spec/` describes the target system.

This document set records the last known implemented, user-visible surface so that:

- spec language is not misread as "already implemented"
- tests can map to explicit supported behavior
- gaps are explicit and actionable

In a docs-only baseline (no implementation artifacts in-repo), treat this ledger as the intended initial reconstruction target until it is updated by the regenerated implementation.

## Status vocabulary (normative)

Use only these status classes in conformance entries:

| Status | Meaning |
|---|---|
| `implemented` | User-visible behavior is reachable and verified by deterministic tests. |
| `partial` | Some required behavior is implemented; explicit gaps remain. |
| `scaffold-only` | Types or structures exist, but behavior is not user-reachable. |
| `planned` | Not implemented; tracked as target scope only. |

Do not use ambiguous labels such as "mostly done", "near complete", or "should work".

## Claim admission gate (normative)

A conformance claim is valid only when all are present:

1. Spec link: exact `/docs/spec/...` path defining expected behavior.
2. Reachable path: user-facing command/key/workflow that triggers the behavior.
3. Verification evidence: deterministic automated tests (and PTY E2E when interactive path matters).
4. Drift handling: user-visible gaps captured in `/docs/reference/LIMITATIONS.md`.

If any item is missing, the claim MUST be recorded as `partial` or `scaffold-only`, not `implemented`.

## Conformance documents (current surface)

| Document | Content |
|----------|---------|
| [CONFORMANCE_MODES_KEYS.md](CONFORMANCE_MODES_KEYS.md) | Modes and keybindings currently supported |
| [CONFORMANCE_EDITING.md](CONFORMANCE_EDITING.md) | Editing semantics (operators, text objects, search behavior) |
| [CONFORMANCE_COMMANDS_TESTING.md](CONFORMANCE_COMMANDS_TESTING.md) | Ex commands and headless/E2E harness surface |

## Update protocol (normative)

When behavior changes:

1. update the relevant conformance entry
2. update `/docs/reference/LIMITATIONS.md` for user-visible gaps
3. run verification commands from `/docs/reference/CI.md`
4. keep conformance statements and test reality synchronized in the same change

## How to use this (recommended)

1. Read the target spec for the area you care about (under `/docs/spec/`).
2. Check this conformance set to confirm what is implemented in the current reconstructed state (or what should be reconstructed first from a docs-only baseline).
3. If there is drift between docs and behavior:
   - update docs and/or implementation, and
   - record user-visible gaps in `/docs/reference/LIMITATIONS.md`.

## Related

- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Spec index: [/docs/spec/README.md](/docs/spec/README.md)
