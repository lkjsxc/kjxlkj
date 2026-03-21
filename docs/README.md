# Documentation Tree TOC

This directory is the authoritative contract tree for `kjxlkj`.

## Global Rules

1. Every docs directory has exactly one `README.md` TOC.
2. Each docs directory has multiple children.
3. Each docs file is under 300 lines.
4. Definitions are canonical in one place and referenced elsewhere.
5. Runtime-file deletion has been completed; contracts in `docs/` are authoritative.
6. Root keep/delete requirements and deletion sequencing are defined in `docs/repository/`.

## Topical Directories

| Directory | Purpose |
| --- | --- |
| [vision/README.md](vision/README.md) | Project intent, setup-first invariants, LLM constraints |
| [product/README.md](product/README.md) | User-visible behavior and policy contracts |
| [architecture/README.md](architecture/README.md) | Runtime, module, routing, and data contracts |
| [containers/README.md](containers/README.md) | Docker Compose and verification container contracts |
| [operations/README.md](operations/README.md) | CLI, testing, quality, and operational checks |
| [repository/README.md](repository/README.md) | Repository layout, docs authoring, and governance |
| [restructuring/README.md](restructuring/README.md) | Deterministic restructuring phases, tests, and full docs coverage matrix |

## Recommended Reading Order

1. [vision/README.md](vision/README.md)
2. [product/flows/setup-flow.md](product/flows/setup-flow.md)
3. [product/surface-map.md](product/surface-map.md)
4. [architecture/runtime/route-topology.md](architecture/runtime/route-topology.md)
5. [architecture/data/README.md](architecture/data/README.md)
6. [containers/README.md](containers/README.md)
7. [operations/README.md](operations/README.md)
8. [repository/README.md](repository/README.md)
9. [restructuring/README.md](restructuring/README.md)
