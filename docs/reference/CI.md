# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles.

## Canonical Location

- CI workflow: `.github/workflows/ci.yml`
- Jobs: docs-integrity → workspace-bootstrap → core-runtime → release-gate

## Baseline State (2025-01-20)

- Active profile target: `Release`.
- `Workspace-bootstrap` profile passes: cargo check + tsc --noEmit + vite build.
- `Core-runtime` profile passes: cargo test --workspace.
- Integration test infrastructure for DB-connected acceptance suites is pending.

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| `Docs-integrity` | documentation changes | deterministic link and structure checks for `/docs` |
| `Workspace-bootstrap` | workspace appears | `Docs-integrity` + workspace compile checks |
| `Core-runtime` | HTTP/API implementation claims | `Workspace-bootstrap` + `cargo test --workspace -- --nocapture` + `docker compose up -d --build` + `/api/readyz` smoke |
| `Realtime` | WS implementation claims | `Core-runtime` + WS subscribe/patch/conflict/replay verification |
| `Librarian-runtime` | librarian feature implementation claims | `Realtime` + `API-AUTO-03`, `API-AUTO-04`, `WS-06`, `E2E-15` |
| `Librarian-small-model` | small-parameter model compatibility claims | `Librarian-runtime` + parser fixture pack with malformed/underspecified XML outputs |
| `Release` | release candidate | all above + perf/ops drills + no high-severity limitations |

## Evidence Rule

CI status claims in ledgers MUST include:

- profile name
- absolute date
- pass/fail signal
- explicit note on open high-severity limitations

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Release gate: [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
