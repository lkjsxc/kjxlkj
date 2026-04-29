# Documentation Canon

`docs/` is the only active canon for product behavior, runtime shape, operations, and repo rules.

## Global Rules

1. Keep one canonical owner for each rule.
2. Keep every docs directory to one `README.md` plus multiple children.
3. Keep every docs file at 300 lines or fewer.
4. Keep every authored source file at 200 lines or fewer.
5. Prefer short declarative bullets over narrative prose.
6. Remove stale contracts instead of preserving conflicting versions.
7. Document exact routes, labels, defaults, and file shapes when they matter.
8. Optimize for LLM retrieval before human ornament.

## Start Paths

- First orientation: [getting-started/README.md](getting-started/README.md)
- Product surface and shared resource model: [product/README.md](product/README.md)
- Media and embedding canon: [product/resources/README.md](product/resources/README.md)
- API and settings contracts: [product/api/README.md](product/api/README.md)
- Storage and schema canon: [architecture/README.md](architecture/README.md)
- Object storage canon: [architecture/storage/README.md](architecture/storage/README.md)
- Full acceptance path: [operations/verification/compose-pipeline.md](operations/verification/compose-pipeline.md)

## Top-Level Sections

- [getting-started/README.md](getting-started/README.md): orientation, compose boot, and verification entrypoints
- [vision/README.md](vision/README.md): product intent and LLM-first documentation rules
- [product/README.md](product/README.md): routes, resources, behavior, and UI contracts
- [architecture/README.md](architecture/README.md): runtime, data, storage, and integration contracts
- [operations/README.md](operations/README.md): deployment, backup, verification, and quality gates
- [repository/README.md](repository/README.md): repo layout, authoring rules, and change workflow

## Recommended Reading Order

1. [vision/purpose.md](vision/purpose.md)
2. [repository/layout/root-layout.md](repository/layout/root-layout.md)
3. [product/resources/resource-kinds.md](product/resources/resource-kinds.md)
4. [product/surface/routes.md](product/surface/routes.md)
5. [product/api/settings.md](product/api/settings.md)
6. [product/resources/embed-rules.md](product/resources/embed-rules.md)
7. [architecture/runtime/stack.md](architecture/runtime/stack.md)
8. [architecture/data/postgres-schema.md](architecture/data/postgres-schema.md)
9. [architecture/storage/object-storage.md](architecture/storage/object-storage.md)
10. [operations/deployment/single-host-compose.md](operations/deployment/single-host-compose.md)
11. [operations/verification/compose-pipeline.md](operations/verification/compose-pipeline.md)
