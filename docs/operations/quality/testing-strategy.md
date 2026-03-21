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

## Compose Verification

```bash
docker compose --profile verify run --rm verify
```
