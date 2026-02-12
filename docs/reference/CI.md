# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles.

## Canonical Location

- `/.github/workflows/ci.yml`

## Baseline State (2026-02-12)

- Active profile target: `Docs-pivot`.
- Runtime profiles are blocked until source workspace is reconstructed.

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| `Docs-integrity` | documentation changes | link validation + structure checks |
| `Workspace-bootstrap` | workspace appears | `Docs-integrity` + `cargo check --workspace` |
| `Core-runtime` | HTTP/API implementation claims | `Workspace-bootstrap` + unit/integration tests |
| `Realtime` | WS implementation claims | `Core-runtime` + WS sync tests |
| `Release` | release candidate | all above + perf/ops evidence and no high-severity limitations |

## Evidence Rule

CI status claims in ledgers MUST include:

- profile name
- absolute date
- pass/fail signal
- explicit note on open high-severity limitations

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Release gate: [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
