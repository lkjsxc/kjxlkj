# Drift Ledger

## Purpose

Track unresolved mismatches between intended contracts and current restructuring docs.

## Severity Levels

- `H`: blocks stage transition
- `M`: requires fix before final acceptance
- `L`: informational and schedulable

## Ledger

| Drift ID | Severity | Scope | Status | Owner | Resolution |
| --- | --- | --- | --- | --- | --- |
| DRIFT-000 | L | initialization | closed | docs-agent | ledger bootstrapped |
| DRIFT-001 | H | compose verification | closed | infra-owner | docker-compose.yml with postgres, app, verify services |
| DRIFT-002 | H | source implementation | closed | impl-agent | full Rust implementation with 23 passing tests |
| DRIFT-003 | M | html-pages | closed | docs-agent | added html-pages.md to product/surface |
| DRIFT-004 | M | postgres-schema | closed | docs-agent | added postgres-schema.md to architecture/data |
| DRIFT-005 | M | password-contract | closed | docs-agent | added password-contract.md to architecture/integrations |

## Update Rule

Any `H` severity row blocks stage closure.
