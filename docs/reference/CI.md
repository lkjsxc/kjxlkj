# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles for docs-only and reconstructed-runtime states.

## Baseline State (2026-02-25)

- Active repository state: docs-first reset.
- Runtime profiles are mandatory once runtime reconstruction begins.

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| `Docs-integrity` | any change | docs presence, link coverage, TODO policy constraints |
| `Wave-build` | each TODO wave | `cargo build --workspace` once runtime manifests exist |
| `Wave-test` | each TODO wave | `cargo test --workspace` + mapped acceptance IDs |
| `Frontend-build` | UI wave touched | strict type-check + frontend production build |
| `Release` | release candidate | all profiles + zero high-severity blockers + full `T0/T1/T2` |

## Mandatory Communication Checks

Release candidate MUST include green results for:

- HTTP contract tests (errors, idempotency, rate limiting)
- WebSocket replay/idempotency/stale-cursor tests
- auth/session/csrf enforcement tests
- degraded-provider fallback tests

## Per-Wave Minimum Commands

- Build gate: `cargo build --workspace`
- Test gate: `cargo test --workspace`
- Acceptance gate: run IDs in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Matrix gate: maintain [/docs/reference/TEST_MATRIX.md](/docs/reference/TEST_MATRIX.md) and [/docs/reference/TODO_TRACE_MATRIX.md](/docs/reference/TODO_TRACE_MATRIX.md)

If runtime is not yet reconstructed, mark the wave as blocked and keep checkboxes unchecked.

## Evidence Rule

Every CI status claim in ledgers MUST include:

- profile name
- absolute date
- pass/fail signal
- command or test identifier

## Fail Conditions

CI MUST fail if any of the following is true:

- acceptance ID exists but has no mapped suite in `TEST_MATRIX.md`
- TODO checkbox completed without evidence row
- `docs/logs/` path appears in repository
- source file length policy is violated after runtime rebuild

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Release gate: [RELEASE.md](RELEASE.md)
