# Architecture

Use this subtree for the non-UI contracts that define how `kjxlkj` runs, stores state, and talks to dependent libraries and formats.

## Read This Section When

- You need the runtime service shape.
- You need the PostgreSQL schema or identifier rules.
- You need the JSON, password, time, or license contracts.

## Child Index

- [runtime/README.md](runtime/README.md): runtime service stack, module boundaries, and route ownership
- [data/README.md](data/README.md): schema, IDs, aliases, search indexing, and consistency rules
- [integrations/README.md](integrations/README.md): protocol and dependency contracts that code must honor

## Start Here

- Runtime overview: [runtime/stack.md](runtime/stack.md)
- Storage overview: [data/postgres-schema.md](data/postgres-schema.md)
- Integration overview: [integrations/json-contract.md](integrations/json-contract.md)
