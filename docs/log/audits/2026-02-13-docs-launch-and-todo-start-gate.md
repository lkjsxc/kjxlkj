# Docs Launch and TODO Start-Gate Audit (2026-02-13)

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Scope

Deterministic reconciliation pass for docs-only baseline:

- restore compose-based docs launch artifacts referenced by guides
- mark completed top-level TODO read/start-gate steps as checked
- run required unchecked-marker scan and decide whether to continue marking TODO rows
- record current source-length check behavior for docs-only baseline

## Changes Applied

1. Restored root docs-launch artifacts:
   - `/Dockerfile`
   - `/docker-compose.yml`
   - `/.dockerignore`
2. Added reconstruction scaffold guide:
   - `/docs/guides/RECONSTRUCTION_BOOTSTRAP.md`
3. Updated guide index:
   - `/docs/guides/README.md`
4. Updated top-level TODO checks (manual line-by-line replacements):
   - `/docs/todo/README.md` start-gate read rows set to `[x]`
   - `/docs/todo/README.md` recursive program open-wave row set to `[x]`
5. Updated improvement log:
   - marked reconstruction-bootstrap idea as completed
   - replaced stale source-length list with docs-only baseline evidence

## Verification Commands

- `docker compose config`
  - pass
- `grep -n "\[ \]" docs/todo/README.md`
  - pass (command executed)
  - result: unchecked rows still present for runtime/API/WS/UI/RBAC/automation/librarian blockers and release gate checks
- `if [ -d src ]; then ...; else echo NO_SOURCE_DIR; fi`
  - result: `NO_SOURCE_DIR`

## Decision

Unchecked rows in `/docs/todo/README.md` remain intentionally open because
`/docs/reference/LIMITATIONS.md` and `/docs/reference/DRIFT_MATRIX.md` still define
high-severity reconstruction blockers in current docs-only baseline.

Therefore, only completed read/start-gate rows are checked in this change.
