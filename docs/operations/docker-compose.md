# Docker Compose Operations

## Profiles

- default: app + postgres
- verify: lint + tests + build + structural checks

## Primary Commands

```bash
docker compose up --build
docker compose --profile verify run --rm verify
```

## Verification Scope

The verify profile runs:

- formatting checks
- lint checks
- unit tests
- build checks
- docs topology checks
- line-limit checks
