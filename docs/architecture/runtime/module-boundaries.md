# Module Boundaries

## Binary and Startup

- CLI entrypoint and command dispatch surface are defined by contracts in `docs/operations/cli/`.
- Web startup wiring and route registration intent are defined by runtime and route contracts in this section.

## Layering

- Core domain logic is isolated from transport and persistence concerns.
- Adapter boundary encapsulates filesystem and database integration.
- Web boundary handles HTTP routing, middleware, and rendering concerns.
- CLI boundary exposes deterministic command interfaces.

## Contract Rule

- Web and CLI surfaces consume shared core logic through explicit adapters.
