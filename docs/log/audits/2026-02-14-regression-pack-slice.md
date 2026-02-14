# 2026-02-14 Regression Pack Slice

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Objective

Restore deterministic regression-pack execution for currently implemented
runtime/UX paths and map proof to `REG-IMP-*`, `REG-USR-*`, and `REG-UX-*`
identifiers where behavior is available.

## Added/Verified Coverage

- Backend deterministic suites remain passing for:
  - note conflict + metadata delete (`REG-IMP-003` related)
  - websocket idempotency replay/conflict (`REG-IMP-002` related)
  - websocket reconnect subscribe `ack_cursor` replay behavior (`REG-IMP-004` related)
  - automation provider/protocol validation baseline
  - saved-view CRUD lifecycle contract
- Frontend deterministic suite (`vitest`) added for:
  - `REG-IMP-001` synced/draft patch-base behavior
  - `REG-USR-002` idempotency fallback without `crypto.randomUUID`
  - `REG-USR-001` non-fatal pre-auth session probe path
  - `REG-USR-003` autosave-first editing path without manual save controls
  - `REG-USR-007` same-cycle title propagation to note list
  - `REG-USR-008` default minimal editor chrome
  - `REG-UX-005` conflict/offline status-rail rendering on autosave failures

## Deterministic Checks

Command:

`npm run -w src/frontend/app test`

Result:

- pass (`test/app.regression.test.ts` + `test/app.regression.more.test.ts`: 8 passed)

Command:

`cargo test -p kjxlkj-server -- --nocapture`

Result:

- pass (6 passed)

Command:

`cargo check --workspace`

Result:

- pass

Command:

`npm run typecheck`

Result:

- pass

Command:

`find src -type f \( -name '*.rs' -o -name '*.ts' -o -name '*.tsx' \) -exec sh -c 'lines=$(wc -l < "$1"); if [ "$lines" -gt 200 ]; then printf "%s:%s\n" "$1" "$lines"; fi' _ {} \;`

Result:

- no output (all runtime source files are <=200 lines)

## Remaining Gap

Full mandatory acceptance-pack closure remains open (`API-*`, `WS-*`, `E2E-*`,
`PERF-*`, `OPS-*` IDs not yet fully reconstructed in this cycle).
