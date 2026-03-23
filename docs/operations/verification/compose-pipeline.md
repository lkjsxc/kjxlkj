# Compose Pipeline Contract

## Verification Command

```bash
docker compose --profile verify run --rm verify
```

## Required Behavior

- Command exits `0` only when all gates pass.
- Any failing command propagates non-zero exit code.
- Pipeline output is deterministic and CI-safe.

## Optional Wrapper

```bash
cargo run --bin kjxlkj -- compose verify
```
