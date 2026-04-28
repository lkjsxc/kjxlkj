# Live Network

Contracts for local compose networking, the app ICE listener, and production edge ownership.

## Child Index

- [topology.md](topology.md): local service map and production edge flow
- [firewall-rules.md](firewall-rules.md): required HTTP, WebSocket, and app ICE ports

## Rules

- The repo-owned app listens on HTTP inside its runtime container.
- The repo-owned app listens on one static UDP port for WebRTC ICE.
- Production TLS terminates at the external edge nginx.
- Production live media uses only the app ICE port.
