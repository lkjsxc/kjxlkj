# Compose Commands Contract

## Build

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml build app verify visual-verify
```

## Start Services

```bash
docker compose up -d
```

Starts `postgres` and `app` services.

## Run Verify Service

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm verify
```

## Run Visual Verify Service

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml run --rm visual-verify
```

## Stop

```bash
docker compose down
```

## Stop with Volume Cleanup

```bash
docker compose -f docker-compose.yml -f docker-compose.verify.yml down -v
```

## View Logs

```bash
docker compose logs -f app
```
