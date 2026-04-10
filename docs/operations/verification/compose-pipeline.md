# Compose Pipeline Contract

## Verification Command

```bash
cp .env.example .env
docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml up -d postgres seaweedfs app
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## Required Behavior

- Rust and docs verification exits `0` only when all code and docs gates pass.
- Visual verification exits `0` only when browser-rendered screenshot checks pass.
- Any failing command propagates non-zero exit code.
- CI uses the same bundle rather than a weaker host-only shortcut.
- The final `down -v` removes PostgreSQL state, SeaweedFS state, and verification caches.

## Acceptance Conditions

1. `verify` passes.
2. `/healthz` responds from the running `app` service.
3. Fresh instances redirect `/` to `/setup`.
4. Setup and login create an admin session.
5. Setup requires the one-time setup code.
6. Write endpoints enforce session authentication.
7. Guest and admin resource/history surfaces expose required navigation and file behaviors.
8. Text responses negotiate compression when requested.
9. Browser-rendered desktop and compact screenshots pass.
