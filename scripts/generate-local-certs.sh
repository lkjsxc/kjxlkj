#!/bin/sh
set -euo pipefail

mkdir -p nginx/ssl

openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
    -keyout nginx/ssl/key.pem \
    -out nginx/ssl/cert.pem \
    -subj "/CN=localhost" \
    -addext "subjectAltName=DNS:localhost,DNS:turn.localhost,IP:127.0.0.1"

echo "Self-signed certificates generated in nginx/ssl/"
