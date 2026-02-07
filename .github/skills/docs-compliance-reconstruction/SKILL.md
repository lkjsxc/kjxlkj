---
name: docs-compliance-reconstruction
description: Implement, rebuild, and verify this repository from documentation with strict conformance and quality gates. Use when requests require production-quality source changes, docs-to-code reconstruction, drift correction, or autonomous one-shot execution following the reconstruction prompt and policy/spec documents.
---

# Docs Compliance Reconstruction

## Objective

Deliver the highest possible implementation quality while keeping behavior aligned with repository documentation.

## Single-Prompt Operation

Treat one user prompt as sufficient to execute end-to-end.

Start by loading these files in this order:

1. `/docs/todo/RECONSTRUCTION_PROMPT.md`
2. `/docs/policy/INSTRUCT.md`
3. `/docs/policy/ROOT_LAYOUT.md`
4. `/docs/spec/README.md`
5. `/docs/reference/CONFORMANCE.md`
6. `/docs/reference/LIMITATIONS.md`
7. `/docs/todo/current/README.md`
8. `/docs/reference/CI.md`

Load additional spec files only for the requested feature area.

If scope is ambiguous, continue autonomously by selecting the most documentation-consistent interpretation. Record non-trivial ambiguity in `/docs/log/proposals/` and carry forward a TODO leaf for the next iteration.

## Non-Negotiable Rules

- Treat `/docs/` as the source of truth over existing code.
- Follow constraints and acceptance criteria from `/docs/todo/RECONSTRUCTION_PROMPT.md`.
- Avoid blocking on user interaction while actionable work remains.
- Keep documentation and implementation synchronized in the same change.
- Keep documentation links rooted at `/docs/...` and never use `../` in docs links.
- Keep changes deterministic, testable, and reproducible.

## Execution Workflow

1. Derive exact acceptance conditions from the relevant docs.
2. Implement the smallest verifiable increment.
3. Add or update tests for every observable behavior and each fixed defect.
4. Re-run verification gates.
5. Update conformance, limitations, logs, and TODO checkboxes whenever behavior or status changes.
6. Continue until all requested scope is complete and verification is green.

## Verification Gates

Run these commands from repo root:

- `python .github/scripts/check_docs_policy.py`
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets`
- `cargo test --workspace`
- `docker build -t kjxlkj:ci .` (when `Dockerfile` exists)

If a gate fails, fix the cause and rerun. If a gate cannot run in the current environment, record a precise limitation and run the closest deterministic substitute.

## Testing Standard

- Add regression tests before or alongside each bug fix.
- Prefer boundary, state-transition, and contract tests for fragile flows.
- Add PTY/E2E tests for user-visible editor behavior where feasible.
- Keep tests hermetic and non-flaky (no dependence on wall-clock timing or network).

## Prompt References

Use `references/reference-prompts.md` for prompt templates that map directly to repository prompt contracts.
