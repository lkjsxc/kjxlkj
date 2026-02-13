# Conformance

Back: [/docs/reference/README.md](/docs/reference/README.md)

This ledger reports only currently verified repository behavior.

## Status Vocabulary

| Status | Meaning |
|---|---|
| `verified` | deterministic evidence exists and no high-severity contradiction is open |
| `partial` | behavior exists but verification is incomplete |
| `blocked` | known user-visible failure or contradiction is open |
| `unverified` | no trustworthy evidence exists in the current baseline |
| `spec-only` | behavior is defined in spec only |

## Current Snapshot (2026-02-13)

High-confidence statement:

- The repository is intentionally in docs-only rebuild baseline state.
- Runtime, build, and deployment artifacts are intentionally absent from this baseline.
- Canonical specs now include explicit incorporation of implementation/user findings
  (`IMP-001..005`, `USR-001..008`) with UX-focused requirements.
- Single-container Docker Compose startup is fully specified in architecture/guides,
  but runnable artifacts remain reconstruction scope.
- Any runtime claim is `spec-only` or `unverified` until reconstruction waves provide
  deterministic evidence.

## Domain Status

| Domain | Canonical Spec | Status | Evidence |
|---|---|---|---|
| Policy and governance model | [/docs/policy/README.md](/docs/policy/README.md) | `verified` | docs-first operating rules and structure constraints are present |
| Findings traceability | [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) | `verified` | all `IMP-*`/`USR-*` findings mapped to normative requirements and tests |
| Deployment contract | [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) | `spec-only` | single-container compose contract and template are documented |
| API contract | [/docs/spec/api/http.md](/docs/spec/api/http.md) | `spec-only` | canonical HTTP surface is defined; no runtime artifact exists in this baseline |
| WS protocol | [/docs/spec/api/websocket.md](/docs/spec/api/websocket.md) | `spec-only` | canonical replay/idempotency/cursor rules are defined; no runtime artifact exists |
| Domain model | [/docs/spec/domain/README.md](/docs/spec/domain/README.md) | `spec-only` | workspace/notes/automation/librarian contracts are documentation-only |
| UI/UX contract | [/docs/spec/ui/README.md](/docs/spec/ui/README.md) | `spec-only` | UX-focused shell/editor/layout behavior is fully specified for rebuild |
| Runtime implementation | [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md) | `unverified` | implementation tree is intentionally absent in docs-only baseline |
| Testing/performance evidence | [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) | `unverified` | no executable test harness is present after reset |

## Conformance Closure Rule

No row may move from `spec-only`/`unverified` to `partial`/`verified` without all:

1. runtime path is implemented and user-reachable
2. deterministic test evidence is archived
3. reference and TODO ledgers are synchronized in the same logical change

## Related

- Open limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- Drift matrix: [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- Findings audit: [/docs/log/audits/2026-02-12-implementation-user-findings.md](/docs/log/audits/2026-02-12-implementation-user-findings.md)
