# Runtime Configuration Contract

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

Defines the required split between non-secret runtime settings and secrets.

## Canonical Sources

| Source | Scope | Secret Allowed |
|---|---|---|
| `data/config.json` | all non-secret runtime configuration | no |
| `.env` | credentials and tokens only | yes |

## Required `data/config.json` Scope

`data/config.json` MUST hold all runtime configuration except secrets.

Categories that MUST be configurable here:

- logging behavior
- HTTP/static hosting behavior
- database pool/runtime tuning (excluding credentials)
- WebSocket runtime tuning
- editor/autosave timing
- automation provider defaults (excluding credentials)
- storage paths and size limits
- feature flags
- non-secret security toggles
- health/readiness path mapping

## Canonical `data/config.json` Schema (Baseline)

```json
{
  "logging": {
    "default_level": "info",
    "json": true,
    "request_log": true
  },
  "server": {
    "bind_addr": "0.0.0.0:8080",
    "static_dir": "./static",
    "request_timeout_ms": 15000,
    "max_request_body_mb": 16,
    "cors_allowed_origins": ["http://127.0.0.1:8080"]
  },
  "database": {
    "app_name": "kjxlkj",
    "max_connections": 20,
    "min_connections": 2,
    "connect_timeout_ms": 5000,
    "idle_timeout_ms": 30000,
    "statement_timeout_ms": 15000
  },
  "websocket": {
    "heartbeat_interval_ms": 10000,
    "client_timeout_ms": 30000,
    "replay_batch_size": 200
  },
  "editor": {
    "autosave_debounce_ms": 800,
    "conflict_retry_limit": 2
  },
  "automation": {
    "default_provider_kind": "lmstudio",
    "base_url": "http://127.0.0.1:1234/v1",
    "model": "local-model",
    "timeout_ms": 30000,
    "max_tokens": 2048,
    "temperature": 0.1,
    "fallback_models": []
  },
  "storage": {
    "attachments_dir": "./data/attachments",
    "backups_dir": "./data/backups",
    "max_attachment_mb": 500
  },
  "features": {
    "dashboard_enabled": false,
    "librarian_enabled": true,
    "saved_views_enabled": true
  },
  "security": {
    "secure_cookies": false,
    "same_site": "lax",
    "csrf_header": "X-CSRF-Token"
  },
  "health": {
    "healthz_path": "/api/healthz",
    "readyz_path": "/api/readyz"
  }
}
```

## Required Secrets (`.env`)

| Variable | Purpose |
|---|---|
| `DATABASE_URL` | PostgreSQL connection string (may include password) |

Other sensitive provider keys (for example OpenRouter API keys) MUST also be
provided by environment/secret store and MUST NOT be committed.

## Startup Rules

1. process MUST load `.env` at startup (if present)
2. process MUST load `data/config.json` (or `KJXLKJ_CONFIG_PATH`)
3. process MUST fail fast on invalid JSON config
4. process MUST fail fast when required secrets are missing
5. no non-secret runtime knob may be sourced from `.env`

## Related

- Runtime startup: [runtime.md](runtime.md)
- Deployment: [deployment.md](deployment.md)
- Root policy: [/docs/policy/ROOT_LAYOUT.md](/docs/policy/ROOT_LAYOUT.md)
