# Transition Scope Contract

## In Scope

- Legalize the persistent-runtime root contract in repository governance docs.
- Replace legacy cleanup-only root assumptions with root classes that support runtime artifacts.
- Keep document-first authority explicit across repository and container contracts.
- Verify links, topology references, and line-limit compliance for touched docs.

## Out of Scope

- Runtime behavior changes without corresponding contract updates.
- Container policy changes that violate compose contracts.
- Ad-hoc root layout changes that bypass governance sequencing.

## Rationale for Document-First Authority

1. `docs/` remains the canonical definition of intended behavior.
2. Persistent runtime artifacts in repository root are valid and expected when contracts require them.
3. Human and LLM operators need one deterministic authority source before changing runtime files.

## Persistent Runtime Root Rule

- Canonical root classes are defined in [../structure/root-layout.md](../structure/root-layout.md).
- Governance and container docs MUST reference that root contract instead of legacy cleanup-only assumptions.
- Any rule that constrains root entries MUST be expressed as a contract update first.

## Executable Intent Preservation Rule

- Required runtime intent MUST be represented in docs before root/runtime entries are added, removed, or renamed.
- Intent preservation uses:
  - behavior contracts in `docs/product/` and `docs/architecture/`
  - operational command contracts in `docs/operations/`
  - repository and governance contracts in `docs/repository/`
  - compose/build contracts in `docs/containers/compose/`

## Deterministic Contract-Update Checklist

After contract updates, all items MUST remain true:

1. Persistent-runtime root contract is explicit in [../structure/root-layout.md](../structure/root-layout.md).
2. Sequencing rule is explicit in [change-policy.md](change-policy.md).
3. Container build/storage/service contracts remain consistent with repository governance.
4. Relevant TOCs link to updated contracts.
5. Link and line-limit sanity checks pass for touched docs.
