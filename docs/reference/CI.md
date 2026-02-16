# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles for docs-only and reconstructed-runtime states.

## Baseline State (2026-02-16)

- Active repository state: docs-only reset.
- Runtime build/test profiles become mandatory as each TODO wave is executed.

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| `Docs-integrity` | any change | docs presence, link coverage, TODO policy constraints |
| `Wave-build` | each TODO wave | `cargo build --workspace` once runtime manifests exist |
| `Wave-test` | each TODO wave | `cargo test --workspace` + wave acceptance IDs |
| `Frontend-build` | UI wave touched | strict type-check + frontend production build |
| `Release` | release candidate | all profiles + zero high-severity blockers |

## Per-Wave Minimum Commands

- Build gate: `cargo build --workspace`
- Test gate: `cargo test --workspace`
- Acceptance gate: run IDs in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

If runtime is not yet reconstructed, mark the wave as blocked and keep checkboxes unchecked.

## Evidence Rule

Every CI status claim in ledgers MUST include:

- profile name
- absolute date
- pass/fail signal
- command or test identifier

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Release gate: [RELEASE.md](RELEASE.md)
