# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles.

## Canonical Location

- CI workflow files are intentionally absent in this repository state.
- Verification is executed through command profiles and recorded in ledgers.

## Baseline State (2026-02-12)

- Active profile target: `Core-runtime`.
- `Realtime` and `Release` remain partial until WS replay/perf/ops gates are complete.

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| `Docs-integrity` | documentation changes | `scripts/check-doc-links.sh` |
| `Workspace-bootstrap` | workspace changes | `Docs-integrity` + `cargo fmt --all --check` + `cargo check --workspace` |
| `Core-runtime` | HTTP/API implementation claims | `Workspace-bootstrap` + `cargo test --workspace -- --nocapture` + `docker compose up -d --build` + `/api/v1/readyz` smoke |
| `Realtime` | WS implementation claims | `Core-runtime` + WS subscribe/patch/conflict verification |
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
