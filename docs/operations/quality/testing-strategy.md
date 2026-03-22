# Testing Strategy Contract

## Unit Tests

- Frontmatter parse/serialize behavior.
- Slug and path conversion behavior.
- Visibility filtering behavior.

## Integration Tests

- Setup lock behavior.
- Login/logout session flow.
- Admin route guard behavior.
- Public/private route behavior.

## UI Contract Coverage

- Server-rendered page contracts for `/`, `/setup`, `/login`, and `/admin`.
- HTMX fragment contracts for open/preview/save/create/rename/delete/toggle admin flows.
- Conflict handling contract: last-write-wins plus visible warning banner.
- JavaScript UX contracts:
  - autosave trigger model (2s debounce, blur, before unload)
  - unsaved-change guard behavior
  - shortcut bindings (`Ctrl/Cmd+S`, `Ctrl/Cmd+N`, `Ctrl/Cmd+Shift+P`, `Ctrl/Cmd+K`)

## Manual Verification Checklist

- Follow [ui-contract-verification.md](ui-contract-verification.md) for deterministic page, HTMX, and JS checks.

## Compose Verification

```bash
docker compose --profile verify run --rm verify
```
