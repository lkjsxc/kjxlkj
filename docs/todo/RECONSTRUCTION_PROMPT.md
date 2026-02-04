# One-shot Reconstruction Prompt (Copilot / Claude)

Back: [/docs/todo/README.md](/docs/todo/README.md)
Paste this prompt into GitHub Copilot (Claude Opus 4.5) to rebuild the repository from docs.

This project follows “All in Docs”: `/docs/` is the source of truth and the system must be reconstructable from docs alone.

<instructions>
You are an autonomous implementation agent operating inside a git repository.

Your task: reconstruct the entire project from documentation (“All in Docs”), by creating the complete source tree, build files, tests, and supporting artifacts.

Treat `/docs/` as the only source of truth. Everything else is derived output.

Assume you are running as GitHub Copilot using Claude Opus 4.5 (or an equivalent coding-capable model).
</instructions>

<one_shot_execution_strategy>
This prompt is designed for a single uninterrupted run.

Do not ask for user input. Do not pause. Do not leave partial work behind.

If something is ambiguous, choose the best default that maximizes compliance with `/docs/` and record the decision in `/docs/reference/IMPLEMENTATION_HISTORY.md`.
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
- Avoid creating giant files: respect the 200-line and max-12-children policies.
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

TODO discipline
- Use `/docs/todo/current/` as your master checklist.
- Actually write checkmarks into the TODO-list document files as you complete items.
- Do not stop until every item in the current TODO iteration is checked off.
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

Version control and traceability
- Commit frequently at meaningful milestones (scaffold, feature slice, refactor, stabilization).
- Keep commits small enough to review and to bisect.

Logging
- Maintain a structured reconstruction log in `/docs/reference/IMPLEMENTATION_HISTORY.md` (append new entries).
- If any file-size policies are violated, record it explicitly there and create a TODO leaf to address it.
</constraints>

<definition_of_done>
You are done only when all of the following are true:

- The repository builds from scratch and all tests pass reliably.
- The implementation matches the canonical spec, or any divergence is recorded in `/docs/reference/`.
- The TODO iteration is fully checked off, including the recursion waves.
- The documentation remains sufficient to delete everything but `/docs/` and reconstruct again.
</definition_of_done>
