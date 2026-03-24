# Wave 091: Conformance and Drift Closure

## Objective

Specify limitation and drift reconciliation procedures.

## Inputs

- [objective.md](objective.md)
- [exit-criteria.md](exit-criteria.md)
- [../../program/operating-model.md](../../program/operating-model.md)
- [../../program/dependency-rules.md](../../program/dependency-rules.md)
- [../../orientation/source-snapshot-mapping.md](../../orientation/source-snapshot-mapping.md)

## Preconditions

- Prior wave in sequence is complete (or this is the first wave).
- Stage prerequisites from [README.md](README.md) are satisfied.
- No unresolved blocker in [../../evidence/drift-ledger.md](../../evidence/drift-ledger.md) prevents execution.

## Procedure

1. Parse all input contracts and extract explicit requirements for this wave.
2. Apply wave-specific documentation updates for `conformance and drift closure`.
3. Cross-link all changed definitions to canonical files.
4. Validate line limits and link integrity for touched files.
5. Record wave outcome in [../../evidence/drift-ledger.md](../../evidence/drift-ledger.md).

## Expected Outputs

- Wave requirements are represented as deterministic markdown contracts.
- All acceptance IDs below are satisfiable without implied context.
- Referenced files contain no ambiguous ownership of definitions.

## Acceptance IDs

- `S09-W091-AC01`: objective-level requirements are fully represented.
- `S09-W091-AC02`: cross-links resolve to canonical documents.
- `S09-W091-AC03`: wave completion checklist is fully satisfied.

## Failure Modes

- `FM-1`: Missing or contradictory requirements across linked files.
- `FM-2`: Broken relative links after wave updates.
- `FM-3`: File-size constraints violated after expansion.

## Recovery Actions

1. Resolve contradictions by selecting one canonical definition and updating inbound references.
2. Repair broken links and rerun link audit checks.
3. Split oversized files into smaller topic-specific files and update TOCs.
4. Re-run mandatory gates from the first gate in [../../checkpoints/gate-checklist.md](../../checkpoints/gate-checklist.md).

## Completion Checklist

- [ ] Procedure completed in full sequence.
- [ ] All three acceptance IDs are satisfied.
- [ ] No unresolved wave-specific blocker remains.
- [ ] Evidence entry recorded for this wave.
