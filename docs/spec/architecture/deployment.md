# Deployment Model

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

## Host Process Contract

Deployment MUST support two processes with explicit lifecycle control:

- PostgreSQL process
- application process

## Startup Contract

1. initialize data directory when absent
2. start DB and wait for readiness
3. run migrations
4. start app server
5. forward shutdown signals and drain in-flight requests

## Required Runtime Artifacts

- `src/` runtime source tree
- `Cargo.toml` and `Cargo.lock`
- optional helper scripts under `scripts/`

## Prohibitions

- Docker artifacts are not part of the canonical baseline.
- Deployment semantics MUST NOT depend on container-only behavior.

## Related

- Runtime model: [runtime.md](runtime.md)
- Operations: [/docs/spec/technical/operations.md](/docs/spec/technical/operations.md)
