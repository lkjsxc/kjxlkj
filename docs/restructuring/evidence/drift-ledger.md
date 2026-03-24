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

## Update Rule

Any `H` severity row blocks stage closure.
