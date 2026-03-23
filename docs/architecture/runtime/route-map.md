# Route Map Contract

## Route to Handler Mapping

- `/healthz` -> health handler.
- `/v1/records` -> list handler.
- `/v1/records/{id}` -> fetch, upsert, delete handlers.

## Guarding Model

- Read handlers are unguarded.
- Write handlers call token guard before mutation.

## Response Model

Handlers return deterministic status codes and JSON payload shapes from product contracts.
