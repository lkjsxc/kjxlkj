# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles for docs and reconstructed runtime states.

## Baseline State (2026-02-16)

- Active repository state: reconstructed runtime.
- Mandatory local gates were executed for this sync cycle.

## Verification Profiles

| Profile | Applies When | Required Checks | Current Status |
|---|---|---|---|
| `Docs-integrity` | any change | docs presence, link coverage, TODO policy constraints | pass |
| `Wave-build` | each TODO wave | `cargo build --workspace` | pass |
| `Wave-test` | each TODO wave | `cargo test --workspace` + wave acceptance IDs | partial (unit tests pass; acceptance IDs incomplete) |
| `Frontend-build` | UI wave touched | strict type-check + frontend production build | pass |
| `Release` | release candidate | all profiles + zero high-severity blockers | blocked (verification gaps) |

## Executed Commands (2026-02-16)

- `cargo build --workspace` → pass
- `cargo test --workspace` → pass (`16` tests)
- `npm --prefix src/frontend/app run check` → pass
- `npm --prefix src/frontend/app run build` → pass

## Per-Wave Minimum Commands

- Build gate: `cargo build --workspace`
- Test gate: `cargo test --workspace`
- Acceptance gate: run IDs in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Evidence Rule

Every CI status claim in ledgers MUST include:

- profile name
- absolute date
- pass/fail signal
- command or test identifier

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Release gate: [RELEASE.md](RELEASE.md)
