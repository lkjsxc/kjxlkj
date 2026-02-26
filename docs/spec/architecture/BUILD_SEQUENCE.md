# Build Sequence Contract

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Deterministic reconstruction order for a full docs-first rebuild.

## Phase Model

| Phase | Goal | Blocking Output |
|---|---|---|
| `P0` | governance and docs integrity | TODO + policy gates pass |
| `P1` | workspace scaffold | compilable manifest and crate roots |
| `P2` | domain and persistence core | note/event/search/auth primitives |
| `P3` | communication layer | HTTP + WebSocket contracts implemented |
| `P4` | frontend integration | root URL and editor workflows live |
| `P5` | hardening and release | full T0/T1/T2 + release gates green |

## Ordered Sequence

1. Execute `S00` governance waves from [/docs/todo/waves/stage-00-pivot-governance/README.md](/docs/todo/waves/stage-00-pivot-governance/README.md).
2. Generate `Cargo.toml` and crate manifests from [workspace-manifest.md](workspace-manifest.md) and [crates.md](crates.md).
3. Create `src/crates/domain/` and `src/crates/db/` first; all service crates depend on these contracts.
4. Regenerate `migrations/` from [/docs/spec/technical/migrations.md](/docs/spec/technical/migrations.md) before any DB integration tests.
5. Implement note/event/workspace/search core from [/docs/spec/domain/README.md](/docs/spec/domain/README.md).
6. Implement communication layer (`http`, `ws`, auth/session/csrf middleware) from [/docs/spec/api/README.md](/docs/spec/api/README.md) and [/docs/spec/security/README.md](/docs/spec/security/README.md).
7. Implement automation and `kjxlkj-agent` runtime from [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md).
8. Implement frontend shell/editor from [/docs/spec/ui/README.md](/docs/spec/ui/README.md).
9. Run per-wave CI gates from [/docs/reference/CI.md](/docs/reference/CI.md) and evidence capture from [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md).
10. Enforce hardening waves (`S10`) before release sign-off.

## Communication Layer Gate (Non-Bypassable)

Before closing `S03`, `S05`, `S06`, or `S07`, all conditions below MUST hold:

- request ID propagation is implemented in HTTP and WebSocket error surfaces
- optimistic concurrency + idempotency behavior is deterministic
- stale cursor replay path returns structured `STALE_CURSOR` details
- auth/session/csrf enforcement is active on every mutating endpoint
- communication acceptance IDs are green in `T0/T1/T2`

## Fail-Fast Rules

- If a phase gate fails, stop and reopen the owning TODO wave.
- No checkbox may be marked complete without linked evidence.
- No later phase can be advanced while earlier-phase blockers remain open.

## Related

- Spec interactions: [SPEC_INTERACTIONS.md](SPEC_INTERACTIONS.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- TODO contract: [/docs/todo/README.md](/docs/todo/README.md)
