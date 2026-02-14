# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles.

## Canonical Location

- Workflow files may be absent in baseline mode.
- Verification claims must be recorded in reference ledgers with evidence lines.

## Baseline State (2026-02-14)

- All in Docs baseline is active.
- Active required profiles: `Docs-integrity`, `Typed-skeleton`, and partial `Core-runtime`.
- `Librarian-runtime` and `Release` remain blocked until high-severity limitations and acceptance gaps close.

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| `Docs-integrity` | documentation changes | link integrity, structure policy, TODO checkbox hygiene |
| `Typed-skeleton` | runtime scaffold claims | `Docs-integrity` + backend compile gate + frontend TS strict gate |
| `Core-runtime` | API/WS/runtime claims | `Typed-skeleton` + runtime integration checks + deterministic frontend regression slice (`vitest`) |
| `Librarian-runtime` | librarian claims | `Core-runtime` + `API-AUTO-03`, `API-AUTO-04`, `WS-06`, `E2E-15` |
| `Release` | release candidate | all above + perf/ops drills + no high-severity limitations |

## Evidence Rule

Each CI claim in ledgers MUST include:

- profile name
- absolute date
- pass/fail signal
- explicit note on open high-severity limitations

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Type safety contract: [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- Release gate: [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
