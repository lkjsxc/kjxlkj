# Proposal: Hard Pivot to Workspace Suite

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Date

2026-02-12

## Decision

Adopt a hard canonical pivot to multi-user workspace-suite contracts while
preserving docs-first governance.

## Scope

- hard-replace canonical API/WS contracts with unversioned paths (`/api`, `/ws`)
- add workspace/project/permissions/automation domain contracts
- define one canonical OpenAPI document at `docs/spec/api/openapi.yaml`

## Rationale

- align product direction with multi-user workspace goals
- eliminate version-labeled contract drift in canonical docs
- keep implementation program deterministic and evidence-driven

## Follow-Up

Execution is tracked in [/docs/todo/waves/README.md](/docs/todo/waves/README.md).
