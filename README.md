# kjxlkj — Document-First Contract Repository

Status: **document-first** repository.

## Repository Model (Normative)

1. The canonical source of target behavior is `docs/`.
2. Documentation defines behavior first; implementation is expected to converge to documented contracts.
3. Runtime and infrastructure artifacts are allowed in repository root when they align with documented contracts.
4. If documentation and runtime behavior disagree, documentation defines the intended target behavior until runtime is aligned.

## Quick Navigation

- Authoritative docs map: [docs/README.md](docs/README.md)
- Vision and invariants: [docs/vision/README.md](docs/vision/README.md)
- Product contracts: [docs/product/README.md](docs/product/README.md)
- Architecture contracts: [docs/architecture/README.md](docs/architecture/README.md)
- Container contracts: [docs/containers/README.md](docs/containers/README.md)
- Operations contracts: [docs/operations/README.md](docs/operations/README.md)
- Repository governance: [docs/repository/README.md](docs/repository/README.md)
- Restructuring program: [docs/restructuring/README.md](docs/restructuring/README.md)

## Start Here (Compact Flow)

1. [Setup-First Contract](docs/vision/setup-first.md) — mandatory startup invariant.
2. [Product Surface Map](docs/product/surface-map.md) — public, auth, and admin route contract.
3. [Setup Flow](docs/product/flows/setup-flow.md) + [Admin Editor Flow](docs/product/flows/admin-editor.md) — behavior sequencing.
4. [Route Topology](docs/architecture/runtime/route-topology.md) + [Content Visibility](docs/architecture/data/content-visibility.md) — enforcement details.
5. [Compose Commands](docs/containers/compose/commands.md) + [Local Verification Runbook](docs/containers/verification/local-runbook.md) — container execution path.
6. [Operations Automation](docs/operations/automation.md) + [Change Policy](docs/repository/governance/change-policy.md) — validation and governance.
7. [Restructuring Program](docs/restructuring/README.md) — deterministic phases, tests, and docs coverage.

## Required Product Behavior

- **Setup-first is mandatory**:
  - If no admin user exists, the setup screen appears first.
  - Login and admin-editor flows must not appear before setup completion.
- After setup completion, normal public/admin flows follow domain contracts in `docs/`.

## LLM Readability and Memory Strategy

To reduce memory pressure and retrieval errors for LLM agents:

- Keep each docs file under 300 lines.
- Use deterministic section names and stable link targets.
- Keep one authoritative definition per rule; other docs should link instead of duplicating text.
- Prefer short bullets with one fact per bullet.

## Repository Scope

- In scope: maintaining clear, deterministic contracts in `docs/` and aligning runtime artifacts with those contracts.
- Out of scope: introducing runtime behavior changes without corresponding contract updates in `docs/`.
