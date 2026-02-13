# Stage 08 Audit: Librarian UX and Static Delivery

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Scope

Stage 08 (`wave-080`/`wave-081`/`wave-082`) delivery for librarian UX and responsive static shell behavior:

- provider/model/strict-mode librarian rule authoring controls
- workspace run launch + run status surfaces in the shell
- per-operation review controls with deterministic accept/reject persistence
- deterministic review audit linkage and workspace-event visibility
- 320px-safe panel behavior and keyboard-first command palette flows

## Implementation Evidence

Changed runtime/UI paths:

- `src/crates/app/kjxlkj-server/static/index.html`
  - added librarian control panel (`provider`, `model`, `strict_mode`) and rule/run lists
  - added run review panel with per-operation accept/reject controls
  - added unresolved-local-draft apply guard for review apply actions
  - added command palette `run-rule` and `review-run` keyboard-first flows
  - preserved menu collapse/restore and responsive single-tree behavior
- `src/crates/app/kjxlkj-server/src/handlers/automation.rs`
  - added `GET /api/automation/runs` list endpoint
  - added `POST /api/automation/rules/{id}/launch` manual run launch endpoint
  - added `POST /api/automation/runs/{id}/review` review/apply endpoint with deterministic validation
  - review apply supports deterministic operation execution summary for accepted operations
- `src/crates/db/kjxlkj-db/src/repos/automation.rs`
  - added `list_runs` query path
  - added `record_run_review` persistence for merged review/apply result payloads
  - emits `automation_run_reviewed` workspace events with deterministic payload counters
- `src/crates/app/kjxlkj-server/tests/automation_run_flow.rs`
  - added coverage for run list, manual launch, review endpoint, invalid decision boundaries, and review event persistence
- `src/crates/app/kjxlkj-server/tests/ui_shell.rs`
  - added librarian panel markup and responsive/command-flow assertion coverage

## Verification Evidence

Executed checks:

1. `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test ui_shell --test automation_run_flow --test ws_flow -- --nocapture`
2. `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test command_workflow -- --nocapture`

Observed results:

- `ui_shell`: pass (`2 passed; 0 failed`)
- `automation_run_flow`: pass (`1 passed; 0 failed`)
- `ws_flow`: pass (`1 passed; 0 failed`)
- `command_workflow`: pass (`1 passed; 0 failed`)

Deterministic acceptance evidence now includes:

- librarian rule authoring and run launch/status APIs are runtime-reachable
- review decisions are validated and persisted with deterministic machine feedback
- review submissions emit `automation_run_reviewed` workspace events for audit visibility
- command palette supports keyboard-first launch/review flow hooks
- responsive librarian surfaces preserve 320px no-mode-fork shell behavior

## Residual Deferred Scope

Stage 08 closes librarian review UX and deterministic review linkage. Deeper release-profile matrix, target-scale performance envelopes, and final release-ledger closure remain Stage 09 scope.
