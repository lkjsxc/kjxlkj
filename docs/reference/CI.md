# CI

Back: [/docs/reference/README.md](/docs/reference/README.md)

Reproducible verification profiles.

## Baseline State (2026-02-15)

- Active repository state: docs-only baseline.
- Runtime CI profiles are currently future-facing targets.

## Verification Profiles

| Profile | Applies When | Required Checks |
|---|---|---|
| `Docs-integrity` | any docs change | markdown lint/link checks, policy constraints, TODO link coverage |
| `Workspace-bootstrap` | runtime scaffold appears | `Docs-integrity` + compile/type gates |
| `Core-runtime` | API/WS implementation claims | integration tests + readiness smoke |
| `Realtime` | WS implementation claims | cursor/replay/idempotency tests |
| `Agent-runtime` | `kjxlkj-agent` claims | prompt JSON load, KV memory carry-over, YOLO safety tests |
| `Release` | release candidate | all profiles + no high-severity limitations |

## Evidence Rule

Every CI status claim in ledgers MUST include:

- profile name
- absolute date
- pass/fail signal
- open high-severity limitation note

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Release gate: [RELEASE.md](RELEASE.md)
