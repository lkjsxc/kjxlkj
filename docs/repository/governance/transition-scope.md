# Transition Scope Contract

## In Scope

- Documentation structure improvements.
- Contract clarity improvements.
- Link and topology correctness checks.
- Defining and enforcing final root keep/delete contracts as documentation.

## Out of Scope

- Reintroducing runtime/container artifacts into repository root.
- Changing deployment/runtime behavior without contract updates.

## Rationale for Docs-Only Authority

1. The repository target state is documentation-contract-only, so `docs/` is the enduring source of truth.
2. Runtime/container code is removed from root; documentation remains sufficient to preserve required behavior contracts.
3. LLM and human operators need deterministic contracts that survive implementation deletion.

## Executable Intent Preservation Rule

- Required runtime intent MUST be represented as contracts in docs before any implementation artifact is deleted.
- Intent preservation uses:
  - behavior contracts in `docs/product/` and `docs/architecture/`
  - operational command contracts in `docs/operations/`
  - repository and governance contracts in `docs/repository/`
- If a deletion would remove the only concrete definition of behavior, deletion is blocked until docs are updated.

## Deterministic Post-Cleanup Checklist

After runtime-file cleanup, all items MUST remain true:

1. Root keep/delete contract is explicit in [../structure/root-layout.md](../structure/root-layout.md).
2. Deletion sequencing rule is explicit in [change-policy.md](change-policy.md).
3. Docs topology and line-limit checks pass.
4. Relevant TOCs link to the updated contracts.
