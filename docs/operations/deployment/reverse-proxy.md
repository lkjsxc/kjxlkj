# Reverse Proxy Deployment

## Scope

- The edge reverse proxy is `nginx` running in Docker Compose.
- It terminates TLS for the web app.
- It performs SNI-based TLS passthrough for coturn.
- It is the only service that binds host ports 80 and 443.

## nginx Configuration

- `nginx/nginx.conf` is mounted read-only into the container.
- The config contains both a `stream` block and an `http` block.
- The `stream` block listens on 443 and routes by SNI.
- The `http` block listens on 8443 internally for HTTPS termination.
- The `http` block also listens on 80 for redirects.

## SNI Stream Routing

```nginx
stream {
    map $ssl_preread_server_name $backend {
        turn.*  coturn:5349;
        turns.* coturn:5349;
        default nginx_https;
    }
    upstream nginx_https { server 127.0.0.1:8443; }
    upstream coturn { server coturn:5349; }
    server {
        listen 443;
        proxy_pass $backend;
        ssl_preread on;
    }
}
```

## HTTP TLS Termination

```nginx
http {
    server {
        listen 8443 ssl;
        ssl_certificate /etc/nginx/ssl/cert.pem;
        ssl_certificate_key /etc/nginx/ssl/key.pem;
        location / {
            proxy_pass http://app:8080;
            proxy_http_version 1.1;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
    server {
        listen 80;
        return 301 https://$host$request_uri;
    }
}
```

## Certificates

### Local Testing

Run `scripts/generate-local-certs.sh` before first boot:

```bash
./scripts/generate-local-certs.sh
```

This creates self-signed certs under `nginx/ssl/` valid for `localhost` and `turn.localhost`.
Browsers will warn about the self-signed certificate; accept the exception for testing.

### Production

Replace `nginx/ssl/cert.pem` and `nginx/ssl/key.pem` with certificates from your provider.
Mount the same certificate files into `coturn/ssl/` so coturn can present the same certificate during TLS passthrough.
For Let's Encrypt, use a certbot sidecar or mount the `/etc/letsencrypt` directory.

## Health Check

- nginx has no custom health endpoint; rely on Docker `HEALTHCHECK` via `curl -fs http://localhost:80/` or TCP port checks.
- In practice, if the app is healthy and nginx starts, the proxy is healthy.
