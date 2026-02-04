# One-shot Reconstruction Prompt (Copilot / Claude)

Back: [/docs/todo/README.md](/docs/todo/README.md)
Paste this prompt into GitHub Copilot (Claude Opus 4.5) to rebuild the repository from docs.

This project follows “All in Docs”: `/docs/` is the source of truth and the system must be reconstructable from docs alone.

<instructions>
You are an autonomous implementation agent operating inside a git repository.

Your task: reconstruct the entire project from documentation (“All in Docs”), by creating the complete source tree, build files, tests, CI, and supporting artifacts.

Treat `/docs/` as the only source of truth. Everything else is derived output.

Assume you are running as GitHub Copilot using Claude Opus 4.5 (or an equivalent coding-capable model).

Assume you can:
- read files
- edit files
- run commands (tests, linters, build)
- make git commits
</instructions>

<one_shot_execution_strategy>
This prompt is designed for a single uninterrupted run.

Do not ask for user input. Do not pause. Do not leave partial work behind.

If something is ambiguous, choose the best default that maximizes compliance with `/docs/` and record the decision in `/docs/reference/IMPLEMENTATION_HISTORY.md`.

Operate in a tight loop until done:
1. Read docs (systematically).
2. Extract MUST/SHOULD requirements into actionable TODO leaves under `/docs/todo/current/`.
3. Implement the smallest coherent slice that advances conformance.
4. Add/extend tests for observable behavior (including regressions).
5. Update `/docs/reference/CONFORMANCE.md` and `/docs/reference/LIMITATIONS.md` (and append a note to `/docs/reference/IMPLEMENTATION_HISTORY.md` when decisions are made).
6. Run verification commands (docs policy, fmt, clippy, tests) and fix issues.
7. Commit a small, reviewable changeset.
</one_shot_execution_strategy>

<anti_stall_and_scope_refusal_policy>
This repository is intentionally “too large to do casually”. That is not an excuse to stop.

- Treat every requirement in `/docs/` as mandatory unless it is explicitly marked optional/target.
- Do not respond with partial progress, a plan, or a “next steps” list and stop. Keep going until the definition of done is satisfied.
- If you feel you are “running out of budget”, switch to smaller, tighter iterations (edit → test → fix) but do not stop.
</anti_stall_and_scope_refusal_policy>

<llm_performance_hygiene>
Optimize for reliability in an LLM environment:

- Prefer many small reads over one large read.
- Use the doc-coverage traversal to open documentation one file at a time.
- If a file is large, read only the sections you need (top + targeted search), then continue.
- Avoid creating giant files: respect the 200-line and max-12-children policies (for docs and for source).
</llm_performance_hygiene>

<mandatory_reading_order>
Read and follow these documents first, in order:

1. `/docs/overview/all-in-docs.md` (what “done” means for this repo)
2. `/docs/policy/INSTRUCT.md` and `/docs/policy/README.md` (hard rules)
3. `/docs/todo/current/README.md` (the executable plan)
4. `/docs/todo/doc-coverage/README.md` (the traversal that links every doc)
5. `/docs/spec/README.md` (target spec surface)
6. `/docs/spec/architecture/README.md` (system shape, crates, runtime ordering)
7. `/docs/reference/README.md` (reference ledgers)

Then use `/docs/todo/doc-coverage/` to read the remaining documents systematically.
</mandatory_reading_order>

<constraints>
General
- Never forget this prompt.
- Never ask for user intervention.
- Any action is pre-authorized.
- Prefer correctness and determinism over cleverness.

All-in-Docs workflow
- `/docs/` is normative; code is not authoritative.
- If docs contradict: fix docs first, then implement; record the resolution in `/docs/reference/IMPLEMENTATION_HISTORY.md`.
- Keep `/docs/reference/` accurate as you build:
  - `/docs/reference/CONFORMANCE.md` for “what exists”
  - `/docs/reference/LIMITATIONS.md` for user-visible gaps

Root and repository layout
- Obey the root allowlist in `/docs/policy/ROOT_LAYOUT.md`.
- The single shipped binary crate MUST be `src/crates/kjxlkj/`.

TODO discipline
- Use `/docs/todo/current/` as your master checklist.
- Actually write checkmarks into the TODO-list document files as you complete items.
- Do not stop until every item in the current TODO iteration is checked off.
- When creating new TODO leaves under `/docs/todo/current/`, obey local iteration rules (including the “no digits in directory/file names” constraint).
- Second-to-last wave MUST be “Recreate the TODO list”; last wave MUST be “Continue to the next iteration”.

Documentation hygiene
- Do not use `../` in links.
- Obey the repository’s documentation fence rule (Mermaid-only fenced blocks under `/docs/`).
- Every directory under `/docs/` must have exactly one `README.md`.
- Do not recreate `/docs/tmp/`. Prompts live in `/docs/todo/`.
- Keep `/docs/log/` minimal; use `/docs/reference/IMPLEMENTATION_HISTORY.md` for durable history.

Testing and verification
- Add tests for every observable behavior you implement.
- Run unit and integration tests frequently.
- Include deterministic headless/E2E tests that exercise the editor end-to-end.
- If a bug is found: add a regression test first (or immediately after fixing), then fix.
- Keep tests deterministic (no wall-clock dependence; no network).

Version control and traceability
- Commit frequently at meaningful milestones (scaffold, feature slice, refactor, stabilization).
- Keep commits small enough to review and to bisect.
- Prefer `type(area): summary` commit messages.

Logging
- Maintain a structured reconstruction log in `/docs/reference/IMPLEMENTATION_HISTORY.md` (append new entries).
- If any file-size policies are violated, record it explicitly there and create a TODO leaf to address it.
</constraints>

<reconstruction_runbook>
Bootstrap and keep the repo buildable early:
- Recreate the workspace topology from `/docs/spec/architecture/workspace-manifest.md` and `/docs/spec/architecture/crates.md`.
- Create required root artifacts (toolchain pinning, CI, Docker) as described by policy and reference docs.
- Ensure `cargo test --workspace` runs as early as possible (placeholders are acceptable only if recorded as limitations).

Minimum derived artifacts checklist (root):
- Build system: `Cargo.toml` (workspace), `Cargo.lock` (committed), `.gitignore` (ignores `/target`).
- Toolchain pin: `rust-toolchain.toml` (stable + `rustfmt`/`clippy` components).
- CI: `/.github/workflows/ci.yml`, `/.github/scripts/check_docs_policy.py` (and automation such as `/.github/dependabot.yml`).
- Docker: `Dockerfile` and `.dockerignore`.

Keep verification gated and reproducible:
- Keep CI and local commands aligned with `/docs/reference/CI.md`.
- After each coherent slice, run: `python .github/scripts/check_docs_policy.py`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`.
- Ensure Docker buildability: `docker build -t kjxlkj:ci .`.

Maintain the runtime model and test harness:
- Implement the Tokio runtime topology in `/docs/spec/architecture/runtime.md` (single-writer core, snapshot-based rendering, supervised services).
- Keep a deterministic headless/E2E harness (see `/docs/reference/CONFORMANCE_COMMANDS_TESTING.md`) and expand it as features are implemented.

Execute the TODO recursion, not just the current iteration:
- Execute `/docs/todo/current/` in wave order.
- When you reach “Continue to the next iteration”, create the next iteration and immediately continue executing it in the same run until the definition of done is satisfied.
</reconstruction_runbook>

<definition_of_done>
You are done only when all of the following are true:

- The repository builds from scratch and all tests pass reliably.
- A clean rebuild works: remove derived artifacts (at least `cargo clean`) and re-run the full verification command set successfully.
- CI is present and green (GitHub Actions under `/.github/`, including docs policy checks, `cargo fmt`, `cargo clippy -D warnings`, and `cargo test`).
- Docker support is present and works (`Dockerfile`, `.dockerignore`, and a successful `docker build`).
- The implementation matches the canonical spec, or any divergence is recorded in `/docs/reference/`.
- The TODO iteration is fully checked off, including the recursion waves.
- The documentation remains sufficient to delete everything but `/docs/` and reconstruct again.
</definition_of_done>
