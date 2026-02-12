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

## Current Snapshot (2026-02-12)

High-confidence statement:

- Repository is intentionally reset to docs-only state for reconstruction.
- Runtime/source/deployment artifacts are intentionally absent.
- Implementation and user findings are captured in audit logs and enforced through updated specs.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance pivot | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | policy and docs-first model are present and linked |
| API v1 contract | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `spec-only` | endpoints specified; runtime intentionally removed |
| WS patch protocol | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `spec-only` | protocol specified with replay/latency expectations |
| Domain model (notes/types/media/search) | [/docs/spec/domain/README.md](/docs/spec/domain/README.md) | `spec-only` | note types and UX-driven feature requirements are specified |
| UI/UX contract | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | `verified` | responsive, autosave, markdown editor, and layout rules specified |
| Deployment/runtime artifacts | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | `unverified` | implementation artifacts intentionally deleted |
| Testing/performance gates | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `spec-only` | acceptance pack expanded; no runtime evidence yet |

## Conformance Closure Rule

No `spec-only` or `unverified` row may move to `verified` without:

1. deterministic test evidence
2. runtime reachability from documented APIs
3. synchronized reference and TODO updates

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Findings audit: [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md)
