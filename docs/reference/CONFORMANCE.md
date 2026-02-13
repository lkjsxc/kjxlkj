# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports only currently verified behavior.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is open |
| `partial` | behavior exists but verification is incomplete |
| `blocked` | known user-visible failure or contradiction is open |
| `unverified` | no trustworthy runtime evidence exists |
| `spec-only` | behavior is defined in spec only |

## Current Snapshot (2026-02-13)

High-confidence statement:

- Repository is intentionally reset to docs-only reconstruction baseline.
- Runtime/source artifacts are intentionally absent.
- Implementation findings (`IMP-*`) and user-reported findings (`USR-*`) are
  integrated in canonical UI/spec/testing docs and remain mandatory regression
  targets for rebuild.
- UX requirements are consolidated as normative requirements in
  [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md).
- Release gate is not open until runtime reconstruction and deterministic
  acceptance evidence are re-established.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | docs-first rules and execution policy are present |
| Findings integration (`IMP-*`/`USR-*`) | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | `verified` | canonical finding-to-requirement mapping exists and links to acceptance IDs |
| UX reconstruction requirements | [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) | `verified` | consolidated UX matrix and closure rules are present |
| API contract | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `spec-only` | runtime currently absent |
| WS protocol | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `spec-only` | runtime currently absent |
| Domain model | [/docs/spec/domain/README.md](/docs/spec/domain/README.md) | `spec-only` | runtime currently absent |
| UI/UX contract execution | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | `spec-only` | runtime currently absent |
| Librarian AI contract | [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) | `spec-only` | runtime currently absent |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `unverified` | runtime currently absent |
| Testing/performance evidence | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `unverified` | no current runtime tests to execute in docs-only baseline |
| Docker Compose docs launch | [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md) | `verified` | `docker compose up -d --build` reaches `healthy`, serves `/docs/README.md`, and shuts down cleanly |

## Conformance Closure Rule

No `spec-only` row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented APIs
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Findings audit: [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md)
- Reset sync audit: [/docs/log/audits/2026-02-13-reconstruction-reset-sync.md](/docs/log/audits/2026-02-13-reconstruction-reset-sync.md)
