# 2026-02-14 Automation Librarian Follow-ups

Back: [/docs/log/improvements/2026/README.md](/docs/log/improvements/2026/README.md)

## Context

Automation/librarian baseline handlers and deterministic tests are restored, but
Stage 06 contract depth remains partial.

## Follow-up Ideas

- Add XML parse-repair retry path coverage for `API-AUTO-04` failure classes.
- Persist run diagnostics (`provider`, `model`, `prompt_hash`, retry attempts)
  with query endpoints for deterministic audit retrieval.
- Add explicit apply-operation safety checks and workspace-scoped guardrails for
  librarian review/approval endpoints.
- Emit librarian automation events on WebSocket stream and validate `WS-06`
  ordering/replay contract.
- Expand deterministic test matrix for launch idempotency and multi-run
  concurrency within a workspace.
