# Change Policy Contract

## Required for Docs Changes

1. Update affected directory TOCs.
2. Keep links valid and relative.
3. Re-run topology and line-limit checks.

## Required for Behavior Changes

- Update vision, product, architecture, and operations contracts consistently.
- Keep canonical definitions singular; replace duplicates with links.

## Deletion Sequencing Rule (Normative)

Runtime/container implementation artifacts MUST be removed only after documentation is improved first.

Required sequence:

1. Define or update contracts so they fully preserve behavior and operational intent.
2. Update TOCs and cross-links so retrieval paths are deterministic.
3. Run validation checks (`docs validate-topology`, `quality check-lines`).
4. Perform deletions in a dedicated cleanup todo (completed for current docs-only state).
5. Re-run validation checks and confirm keep/delete root contract.

If step 1-3 are incomplete, deletion work is non-compliant and MUST stop.

## Deterministic Deletion-Readiness Checklist

Mark each item true before deleting implementation artifacts:

1. Final root keep-set is documented.
2. Delete-set is documented and explicit.
3. Executable intent is documented in canonical contracts.
4. All links in touched docs resolve.
5. Topology check passes.
6. Line-limit check passes.
