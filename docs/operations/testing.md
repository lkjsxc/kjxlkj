# Testing Strategy

## Unit Tests

- frontmatter parse/serialize behavior
- slug and path conversion behavior
- private visibility filtering behavior

## Integration Tests

- setup flow lock behavior
- login/logout session flow
- admin route guard behavior
- public/private route behavior

## Compose Verification

All checks run through Docker Compose:

```bash
docker compose --profile verify run --rm verify
```

## CI

GitHub Actions executes the same command in `.github/workflows/verify.yml`.
