# Operations Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Health and Readiness

- `/api/healthz` checks process liveness.
- `/api/readyz` checks DB connectivity and migration compatibility.

## Backup and Recovery

| Operation | Requirement |
|---|---|
| SQL backup | callable via admin API and auditable |
| restore drill | MUST prove functional parity after restore |
| markdown export | MUST emit deterministic workspace-scoped artifacts |

## Observability

- structured logs MUST include `request_id`, `user_id`, `workspace_id`, and
 principal/session context
- error logs MUST include stable error code
- critical jobs (export/backup/automation) MUST emit start/finish/failure events
- librarian runs MUST record provider kind, model, prompt hash, parse warnings,
  and operation apply/reject counts

## Provider Operations

- Provider credentials MUST be managed through runtime secret configuration.
- OpenRouter egress SHOULD use explicit outbound allowlist and timeout policy.
- LM Studio local mode SHOULD include health probes and restart-safe retry policy.
- Provider outages MUST degrade gracefully to failed run records, not silent drops.

## Related

- Export model: [/docs/spec/domain/export.md](/docs/spec/domain/export.md)
- Automation model: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- Librarian technical contract: [librarian-agent.md](librarian-agent.md)
- CI/Release: [/docs/reference/CI.md](/docs/reference/CI.md)
