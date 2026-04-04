# Compose Pipeline Contract

## Verification Command

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres app
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Required Behavior

- Rust/docs verification exits `0` only when all code and docs gates pass.
- Visual verification exits `0` only when browser-rendered screenshot checks pass.
- Any failing command propagates non-zero exit code.
- Pipeline output is deterministic and CI-safe.
- CI uses the same bundle rather than a weaker host-only shortcut.
- The pipeline keeps compose health checks enabled instead of replacing them with ad hoc shell sleeps.

## Optional Wrapper

```bash
cargo run --bin kjxlkj -- compose verify
```

- The wrapper runs the same full compose bundle.
