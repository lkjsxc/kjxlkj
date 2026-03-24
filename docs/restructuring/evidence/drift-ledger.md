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
| DRIFT-001 | H | compose verification | open | infra-owner | missing compose config blocks required compose gate |
| DRIFT-002 | H | source implementation | open | impl-agent | no Rust source code exists yet |
| DRIFT-003 | M | html-pages | closed | docs-agent | added html-pages.md to product/surface |
| DRIFT-004 | M | postgres-schema | closed | docs-agent | added postgres-schema.md to architecture/data |
| DRIFT-005 | M | password-contract | closed | docs-agent | added password-contract.md to architecture/integrations |

## Update Rule

Any `H` severity row blocks stage closure.
