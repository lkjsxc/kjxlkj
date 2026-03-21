# kjxlkj — Documentation Contract Repository

Status: **documentation-contract-only** repository.

## Repository Contract (Normative)

1. The canonical source of truth is `docs/`.
2. Root keep-set is fixed to `.gitignore`, `LICENSE`, `README.md`, and `docs/`.
3. Runtime and infrastructure artifacts are intentionally removed from repository root.
4. If documentation and runtime behavior disagree, documentation contracts define the intended target behavior.

## Quick Navigation

- Authoritative docs map: [docs/README.md](docs/README.md)
- Vision and invariants: [docs/vision/README.md](docs/vision/README.md)
- Product contracts: [docs/product/README.md](docs/product/README.md)
- Architecture contracts: [docs/architecture/README.md](docs/architecture/README.md)
- Container contracts: [docs/containers/README.md](docs/containers/README.md)
- Operations contracts: [docs/operations/README.md](docs/operations/README.md)
- Repository governance: [docs/repository/README.md](docs/repository/README.md)

## Start Here (Compact Flow)

1. [Setup-First Contract](docs/vision/setup-first.md) — mandatory startup invariant.
2. [Product Surface Map](docs/product/surface-map.md) — public, auth, and admin route contract.
3. [Setup Flow](docs/product/flows/setup-flow.md) + [Admin Editor Flow](docs/product/flows/admin-editor.md) — behavior sequencing.
4. [Route Topology](docs/architecture/runtime/route-topology.md) + [Content Visibility](docs/architecture/data/content-visibility.md) — enforcement details.
5. [Compose Commands](docs/containers/compose/commands.md) + [Local Verification Runbook](docs/containers/verification/local-runbook.md) — container execution path.
6. [Operations Automation](docs/operations/automation.md) + [Change Policy](docs/repository/governance/change-policy.md) — validation and governance.

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

- In scope: maintaining clear, deterministic contracts in `docs/`.
- Out of scope: storing runtime implementation artifacts in repository root.
