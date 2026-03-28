# Compose Commands Contract

## Build

```bash
docker compose build app verify visual-verify
```

## Start Services

```bash
docker compose up -d
```

Starts `postgres` and `app` services.

## Run Verify Profile

```bash
docker compose --profile verify run --rm verify
```

## Run Visual Verify Profile

```bash
docker compose --profile verify run --rm visual-verify
```

## Stop

```bash
docker compose down
```

## Stop with Volume Cleanup

```bash
docker compose down -v
```

## View Logs

```bash
docker compose logs -f app
```
