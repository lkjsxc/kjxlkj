# Compose Pipeline Contract

## Verification Command

```bash
docker compose build app verify visual-verify
docker compose up -d postgres app
docker compose --profile verify run --rm verify
docker compose --profile verify run --rm visual-verify
docker compose down -v
```

## Required Behavior

- Rust/docs verification exits `0` only when all code and docs gates pass.
- Visual verification exits `0` only when browser-rendered screenshot checks pass.
- Any failing command propagates non-zero exit code.
- Pipeline output is deterministic and CI-safe.
- CI uses the same bundle rather than a weaker host-only shortcut.

## Optional Wrapper

```bash
cargo run --bin kjxlkj -- compose verify
```
