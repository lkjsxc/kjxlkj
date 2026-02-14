# 2026-02-14 HTTP + WS + Security Contract Slice

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Objective

Restore reachable HTTP and WebSocket contracts with deterministic security guards
(auth/session/csrf/rbac) and attach executable proof.

## Implementation Coverage

- modularized runtime into route/handler/state/auth/error/ws modules
- implemented setup/login/logout/session behavior with setup lockout
- implemented role-guarded user/workspace/project and notes route surface
- implemented note conflict semantics (`409`) and metadata delete `204`
- implemented `/ws` auth-checked handshake and core message flow:
  - `subscribe_note`
  - `subscribe_workspace`
  - `ack` stale-cursor error path
  - `apply_patch` conflict + idempotency replay-safe commit identity
- wired remaining non-core endpoints to deterministic reachable placeholders

## Deterministic Checks

### Build/type gates

Command:

`cargo check --workspace`

Result:

- pass

Command:

`npm run typecheck`

Result:

- pass

### Contract tests

Command:

`cargo test -p kjxlkj-server -- --nocapture`

Result:

- pass (`3 passed; 0 failed`)

Evidence from tests:

- `api_auth_setup_lock_and_session_flow`: setup lockout + session lifecycle
- `api_note_conflict_and_metadata_delete_contract`: `409` conflict and metadata
  delete `204`
- `ws_patch_conflict_and_idempotency_replay`: `patch_rejected` conflict and
  replay-safe duplicate idempotency key commit identity

### Source line-limit audit

Command:

`find src -type f \( -name '*.rs' -o -name '*.ts' -o -name '*.tsx' \) -exec sh -c 'lines=$(wc -l < "$1"); if [ "$lines" -gt 200 ]; then printf "%s:%s\n" "$1" "$lines"; fi' _ {} \;`

Result:

- no output (no runtime source file exceeds 200 lines)

## Remaining Gaps

- frontend shell/editor UX requirements remain incomplete
- automation/librarian execution semantics remain incomplete
- full acceptance pack (`REG-IMP-*`, `REG-USR-*`, `REG-UX-*`) remains open
- final structure-completion contract remains open
