# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles.

## Canonical Location

- CI workflow files are intentionally absent in this repository state.
- Verification is defined as command profiles and evidence ledgers.

## Baseline State (2026-02-13)

- Repository is in docs-only rebuild baseline.
- Only `Docs-integrity` is directly executable in this baseline.
- Runtime profiles are blocked until reconstruction restores source/runtime artifacts.

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| `Docs-integrity` | documentation changes | deterministic link/structure checks for `/docs` and root docs |
| `Workspace-bootstrap` | source tree reconstructed | `Docs-integrity` + workspace compile checks |
| `Core-runtime` | HTTP/API implementation claims | `Workspace-bootstrap` + integration tests + `docker compose up --build` + `/api/readyz` smoke |
| `Realtime` | WS implementation claims | `Core-runtime` + replay/idempotency/cursor checks |
| `Librarian-runtime` | librarian feature claims | `Realtime` + `API-AUTO-03`, `API-AUTO-04`, `WS-06`, `E2E-15` |
| `Librarian-small-model` | small-model compatibility claims | `Librarian-runtime` + malformed/underspecified XML parser fixture pack |
| `Release` | release candidate | all above + perf/ops drills + no open high-severity limitations |

## Evidence Rule

CI status claims in ledgers MUST include:

- profile name
- absolute date
- pass/fail signal
- explicit note on open high-severity limitations

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Release gate: [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
