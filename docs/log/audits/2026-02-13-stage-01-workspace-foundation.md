# Audit: Stage 01 Workspace Foundation Closure

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Date

2026-02-13

## Scope

Closure evidence for:

- Wave 010: Runtime and workspace bootstrap
- Wave 011: Auth and session baseline
- Wave 012: RBAC and membership controls

## Implementation Summary

- bootstrapped Rust Cargo workspace with runtime crates under `src/crates/`
- wired app runtime (`kjxlkj-server`) with Actix startup, SQLx Postgres pool, and migration bootstrap
- added `kjxlkj-workspace` and `kjxlkj-rbac` crates and integrated domain-level authz checks
- added Stage 01 Postgres migration for users/workspaces/memberships/projects/sessions/security events
- implemented `GET /api/healthz`, `GET /api/readyz` (and root aliases)
- implemented setup/login/logout/session endpoints with secure cookie + CSRF semantics
- implemented user list/create/role update and workspace membership list/upsert APIs
- implemented auditable security event emission for role and membership mutations

## Touched Implementation Paths

- `/Cargo.toml`
- `/src/crates/app/kjxlkj-server/`
- `/src/crates/auth/kjxlkj-auth/`
- `/src/crates/db/kjxlkj-db/`
- `/src/crates/domain/kjxlkj-domain/`
- `/src/crates/rbac/kjxlkj-rbac/`
- `/src/crates/workspace/kjxlkj-workspace/`

## Deterministic Checks

### Check 1: Workspace compile baseline

Command:

```bash
cargo check --workspace --tests
```

Result: pass.

Proof:

```text
Checking kjxlkj-db v0.1.0
Checking kjxlkj-server v0.1.0
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
```

### Check 2: Wave 010 migration + ownership invariants

Commands:

```bash
TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/kjxlkj_test cargo test -p kjxlkj-db --test migration_smoke -- --nocapture
TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/kjxlkj_test cargo test -p kjxlkj-workspace --test ownership_invariants -- --nocapture
```

Result: pass.

Proof:

```text
test migration_and_ready_queries_smoke ... ok
test result: ok. 1 passed; 0 failed

test workspace_owner_and_membership_boundaries ... ok
test result: ok. 1 passed; 0 failed
```

### Check 3: Wave 010 startup smoke (`/api/healthz`, `/api/readyz`)

Result: pass.

Proof:

```text
{"status":"ready"}
{"status":"ok"}
```

### Check 4: Wave 011 API-AUTH acceptance + negative auth paths

Result: pass.

Proof subset:

```text
PASS | API-AUTH-01 setup register first owner | status=201
PASS | API-AUTH-01 setup lockout second attempt | status=409
PASS | Auth negative wrong password | status=401
PASS | API-AUTH-02 session payload | status=200
PASS | API-AUTH-02 logout | status=204
PASS | API-AUTH-02 session revoked after logout | status=401
```

### Check 5: Wave 012 permission matrix + forbidden paths

Result: pass.

Proof subset:

```text
PASS | Owner upserts workspace member admin | status=200
PASS | Admin can upsert workspace member | status=200
PASS | Editor forbidden workspace member upsert | status=403
PASS | Viewer forbidden workspace member upsert | status=403
PASS | Admin forbidden global role update | status=403
PASS | Owner global role update | status=200
ALL_STAGE01_API_CHECKS_PASSED
SECURITY_EVENTS_COUNT=10
```

## Conclusion

Stage 01 wave objectives are implemented with deterministic runtime evidence and test outputs. Remaining program waves (Stage 02+) continue in order from this baseline.