# Root Layout Contract

## Goal

- Define the canonical persistent-runtime repository root.
- Keep document-first authority explicit while legalizing runtime artifacts at root.

## Document-First Authority (Normative)

1. `docs/` is the canonical source of target behavior and operational rules.
2. Runtime and infrastructure entries at repository root are valid only when they align with contracts in `docs/`.
3. If runtime shape and docs differ, docs define the intended target state until implementation is aligned.

## Canonical Root Classes (Normative)

### Class A — Required Governance Entries (tracked)

The repository root MUST include:

1. `.gitignore`
2. `LICENSE`
3. `README.md`
4. `docs/`

### Class B — Canonical Runtime Entries (tracked when runtime stack is enabled)

The repository root SHOULD persist these entries for the compose/runtime workflow:

1. `Dockerfile`
2. `docker-compose.yml`
3. `migrations/`
4. `Cargo.toml`
5. `Cargo.lock`
6. `src/`
7. `templates/`
8. `static/`
9. `content/`
10. `tests/` (if runtime tests are maintained)

### Class C — Runtime State Entries (local, not tracked)

- Host-mounted runtime state MUST be rooted under `./data/`.
- PostgreSQL durable state path MUST be `./data/postgres`.
- `./data/` MUST remain ignored by git.

## Root Change Rule

- Adding, removing, or renaming root entries MUST follow sequencing in [../governance/change-policy.md](../governance/change-policy.md).
- Container-specific root requirements are defined in [../../containers/compose/build-storage-contract.md](../../containers/compose/build-storage-contract.md).

## Deterministic Verification Checklist

1. Run `git ls-tree --name-only HEAD | sort` and confirm all Class A entries are present.
2. If compose/runtime workflow is declared active, confirm Class B entries required by container contracts are present.
3. Confirm `.gitignore` still ignores `data/`.
4. Confirm touched links resolve and docs line limits remain compliant.
