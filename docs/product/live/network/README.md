# Live Network

Contracts for local compose networking and production edge ownership.

## Child Index

- [topology.md](topology.md): local service map and production edge flow
- [firewall-rules.md](firewall-rules.md): required HTTP, WebSocket, and TURN ports

## Rules

- The repo-owned app listens on HTTP inside its runtime container.
- Production TLS terminates at the external edge nginx.
- Production TURN belongs to edge infrastructure, not the app container.
