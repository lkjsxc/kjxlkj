# Testing Strategy Contract

## Unit Tests

- Frontmatter parse/serialize behavior.
- Draft placeholder title/slug generation.
- Slug validation and path conversion.
- CLI topology/line-limit/term-scan logic.

## Integration Tests

- Setup-first lifecycle and setup lock.
- Password-only login/logout session lifecycle.
- Private-by-default article visibility.
- Inline edit save flow on article page.
- Article navigation and history routes.
- Admin create/rename/delete/toggle flows.
- Trash and settings flows.

## UI Contract Coverage

- `/admin` is dashboard only (no dedicated editor pane).
- `/article/{slug}` exposes inline editor only for admin.
- Save/preview buttons are absent.
- Private toggle is above body in inline form.
- Last updated date is displayed.
- Previous/next links are present.
- History page and restore path are present.

## Compose Verification

```bash
docker compose --profile verify run --rm verify
```
