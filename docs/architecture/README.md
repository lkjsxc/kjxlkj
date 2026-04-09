# Architecture

Use this subtree for non-UI contracts that define runtime shape, persistence, and dependent systems.

## Read This Section When

- You need the runtime service shape.
- You need PostgreSQL or object-storage schema rules.
- You need JSON, password, time, or license contracts.

## Child Index

- [runtime/README.md](runtime/README.md): runtime service stack, module boundaries, and route ownership
- [data/README.md](data/README.md): schema, IDs, aliases, search indexing, and consistency rules
- [storage/README.md](storage/README.md): S3-compatible object storage contracts for media
- [integrations/README.md](integrations/README.md): protocol and dependency contracts that code must honor

## Start Here

- Runtime overview: [runtime/stack.md](runtime/stack.md)
- Data overview: [data/postgres-schema.md](data/postgres-schema.md)
- Storage overview: [storage/object-storage.md](storage/object-storage.md)
