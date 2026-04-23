# Coturn Setup

## Scope

- `coturn` provides STUN, TURN, and TURN-over-TLS (TURNS) for WebRTC ICE.
- It runs as a Docker Compose service.
- Its TLS port is reachable only through nginx SNI passthrough on host 443.
- Its UDP and TCP ports may be exposed directly on the host for best performance.

## Configuration

- `coturn/turnserver.conf` is generated from `coturn/turnserver.conf.template` at container startup.
- Environment variables are substituted via `envsubst` in the entrypoint script.

## Required Environment Variables

- `PUBLIC_HOST` — the public hostname used in ICE server URLs and the TURN realm.
- `TURN_STATIC_AUTH_SECRET` — the shared secret for static authentication.
- `EXTERNAL_IP` — the public IP address coturn advertises. Use `auto` to let coturn detect it.

## turnserver.conf Template

```
listening-port=3478
tls-listening-port=5349
listening-ip=0.0.0.0
external-ip=${EXTERNAL_IP}
min-port=49152
max-port=65535
fingerprint
lt-cred-mech
static-auth-secret=${TURN_STATIC_AUTH_SECRET}
realm=${PUBLIC_HOST}
cert=/etc/coturn/ssl/cert.pem
pkey=/etc/coturn/ssl/key.pem
no-cli
no-tlsv1
no-tlsv1_1
```

## Docker Compose Service

```yaml
  coturn:
    image: coturn/coturn:alpine
    ports:
      - "3478:3478/udp"
      - "3478:3478/tcp"
    expose:
      - "5349"
    volumes:
      - ./coturn/turnserver.conf:/etc/coturn/turnserver.conf:ro
      - ./nginx/ssl:/etc/coturn/ssl:ro
    environment:
      PUBLIC_HOST: ${PUBLIC_HOST}
      TURN_STATIC_AUTH_SECRET: ${TURN_STATIC_AUTH_SECRET}
      EXTERNAL_IP: ${EXTERNAL_IP:-auto}
```

## Health Check

- Coturn does not expose an HTTP health endpoint.
- Verify functionality by checking that the TURN port responds:
  ```bash
  nc -z -u localhost 3478
  ```
- Verify TLS passthrough by checking the nginx stream route.

## Logs

- Coturn logs to stdout by default.
- Use `docker compose logs coturn` to inspect authentication failures or relay activity.
