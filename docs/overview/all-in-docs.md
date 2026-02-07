# 「All in Docs」

Back: [/docs/overview/README.md](/docs/overview/README.md)
The documentation is the project.

## Contract (normative)

This repository is intentionally designed so that:

1. `/docs/` is the **single source of truth**.
2. The system is **reconstructable from docs alone**.
3. Code and build artifacts are **derived outputs** that may be deleted and regenerated.

When there is a conflict, prefer fixing the docs and regenerating the implementation.

## What “reconstructable from docs” means

The docs MUST be sufficient to recreate, without needing any existing code:

- repository layout (what directories and files exist)
- Cargo workspace topology (crate list and responsibilities)
- runtime ordering model (input → core → snapshots → render; services supervision)
- data model and invariants (buffers, cursors, modes, selections, registers, marks)
- user-facing behaviors (commands, keybindings, UI rules)
- test strategy, including deterministic/headless E2E
- conformance and limitation tracking rules

The goal is not “perfect prose”; the goal is a **complete rebuild spec**.

## How this repo is intended to be used

### For humans

- Use `/README.md` as an entrypoint, then switch to [/docs/README.md](/docs/README.md).
- Treat `/docs/spec/` as the target requirements.
- Treat `/docs/reference/` as the implementation status ledger.

### For LLMs (primary)

An implementation agent SHOULD:

1. Read policy first: [/docs/policy/INSTRUCT.md](/docs/policy/INSTRUCT.md)
2. Follow the active plan: [/docs/todo/current/README.md](/docs/todo/current/README.md)
3. Use the one-shot rebuild prompt: [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md)

## Project history (context)

This repo intentionally started as **documentation-only**.

An implementation was later generated via **GitHub Copilot** using **Claude Opus 4.5**. The workflow assumes future rebuilds may repeat this pattern: delete derived artifacts and regenerate from `/docs/`.

This history is informative, not authoritative; `/docs/` remains the source of truth.

## Required documentation “loops”

The docs are not static; they encode the maintenance workflow:

### Spec → Implement → Verify

- Specs live in `/docs/spec/`.
- When implementation is created or changed, update:
  - `/docs/reference/` (what is implemented)
  - `/docs/log/` proposals/audits when non-normative work products are useful for iteration

### Drift management

- `/docs/reference/CONFORMANCE.md` records what exists.
- `/docs/reference/LIMITATIONS.md` records user-visible gaps and caveats.
- `/docs/todo/` drives the next implementation cycle.

## Related

- Docs index: [/docs/README.md](/docs/README.md)
- Policy: [/docs/policy/README.md](/docs/policy/README.md)
- Spec index: [/docs/spec/README.md](/docs/spec/README.md)
- Current conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
