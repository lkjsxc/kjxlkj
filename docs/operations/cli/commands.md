# CLI Commands Contract

## Serve

- `serve` runs HTTP server (`kjxlkj-app` binary).

## Docs and Quality

- `docs validate-topology`
- `docs validate-terms`
- `quality check-lines`

## Container Verification

- `compose verify`
  - Runs convergence checks in this order:
    1. `docs validate-topology`
    2. `docs validate-terms`
    3. `quality check-lines`
    4. `docker compose --profile verify run --rm verify`
  - Emits one JSON line per step plus a final summary.

## Notes

- CLI outputs deterministic JSON lines.
- Non-zero exit code means contract failure.
