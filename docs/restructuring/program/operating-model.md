# Operating Model

## Program Shape

- Total stages: 11 (`S00` to `S10`).
- Waves per stage: 3.
- Total waves: 33.
- Stage closure requires all wave completion checklists plus stage exit criteria.

## Execution Surfaces

- Planning and sequencing: [sequencing-rules.md](sequencing-rules.md)
- Dependency management: [dependency-rules.md](dependency-rules.md)
- Risk handling: [risk-model.md](risk-model.md)
- Gates and evidence: [../checkpoints/README.md](../checkpoints/README.md), [../evidence/README.md](../evidence/README.md)

## Actor Model (LLM-First)

- **Planner agent** maintains stage ordering and scope boundaries.
- **Author agent** edits docs under fixed templates.
- **Verifier agent** runs structure, link, and gate audits.
- **Recorder agent** writes deterministic evidence and drift state.

## Completion Rule

Program completion requires all stages marked complete and no unresolved blockers in [../evidence/drift-ledger.md](../evidence/drift-ledger.md).
