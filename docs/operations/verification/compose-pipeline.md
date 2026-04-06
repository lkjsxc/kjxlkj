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
- The final `down -v` removes PostgreSQL runtime state and verification caches.

## Acceptance Conditions

1. `verify` passes.
2. `/healthz` responds from the running `app` service.
3. Fresh instances redirect `/` to `/setup`.
4. Setup and login create an admin session.
5. Write endpoints enforce session authentication.
6. Guest and admin note/history surfaces expose the required navigation controls.
7. Browser-rendered desktop and compact screenshots pass.

## Optional Wrapper

```bash
cargo run --bin kjxlkj -- compose verify
```

- The wrapper runs the same full compose bundle.
- Use [compose-verification-services.md](compose-verification-services.md) for the deeper service contract.
