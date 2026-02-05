# Proposal: Reconstruction Target Selection

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Context

The repository follows an “All in Docs” contract:

- `/docs/spec/` defines the long-term target system.
- `/docs/reference/CONFORMANCE.md` defines the current implemented surface (the "what exists" ledger).

Iteration 36 requires selecting a reconstruction target so implementation and verification work is bounded and test-gated.

## Decision

Select the **current surface** defined by `/docs/reference/CONFORMANCE.md` as the reconstruction target for Iteration 36.

## Rationale

| Consideration | Notes |
|---|---|
| Bounded scope | The full target spec is large; the conformance surface is a tractable slice. |
| Test gating | The conformance surface is intended to be provable via automated tests, including headless and PTY-driven E2E. |
| Drift control | Divergence from `/docs/spec/` remains explicit via `/docs/reference/LIMITATIONS.md`. |
| Recursion-friendly | The current surface can be reconstructed repeatedly while expanding conformance in future iterations. |

## Consequences

- Any newly implemented observable behavior MUST update conformance and limitations documents.
- Any non-implemented target behavior MUST remain explicit (limitations + TODO leaves), not implicit.
- Verification infrastructure (docs policy checks, CI, Docker, headless/E2E harness) becomes mandatory for treating the repo as shippable.

## Follow-ups

- Reconstruct the missing derived artifacts required by policy and reference docs (CI config, docs policy checks, Docker artifacts, toolchain pinning).
- Implement the headless runner defined by `CONFORMANCE_COMMANDS_TESTING.md` and add deterministic tests that exercise it.
- Add PTY-driven E2E regression tests for the highest-priority UX defects recorded in `LIMITATIONS.md`.
