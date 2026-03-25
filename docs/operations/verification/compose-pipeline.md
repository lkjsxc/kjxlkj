# Compose Pipeline Contract

## Verification Command

```bash
docker compose --profile verify run --rm verify
docker compose --profile verify run --rm visual-verify
```

## Required Behavior

- Rust/docs verification exits `0` only when all code and docs gates pass.
- Visual verification exits `0` only when browser-rendered screenshot checks pass.
- Any failing command propagates non-zero exit code.
- Pipeline output is deterministic and CI-safe.

## Optional Wrapper

```bash
cargo run --bin kjxlkj -- compose verify
```
