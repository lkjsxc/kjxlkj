# Gate Checklist

## Ordered Gates

1. Tree structure audit: see [doc-structure-audit.md](doc-structure-audit.md).
2. Link integrity audit: see [link-integrity-audit.md](link-integrity-audit.md).
3. Docs line-limit audit (`<=300` lines per markdown file).
4. Repository quality gate contracts from [../../operations/quality/gates.md](../../operations/quality/gates.md) when executable.
5. Compose verification protocol from [compose-verification-protocol.md](compose-verification-protocol.md).

## Restart Rule

Any failure restarts execution at gate 1 after remediation.

## Evidence Rule

Each gate result must be recorded as `pass`, `fail`, or `blocked` in [../evidence/final.md](../evidence/final.md).
