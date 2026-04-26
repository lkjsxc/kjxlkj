# Live Broadcast

Use this subtree for the `/live` public broadcast page, its WebRTC signaling contract, and external connectivity expectations for remote viewers.

## Read This Section When

- You need the intended admin screen, camera, quality, and microphone workflow.
- You need the public viewer behavior for the live page.
- You need the WebSocket signaling and ICE-server contract.
- You need local app network behavior or external ICE-server requirements.

## Child Index

### Page and Lifecycle
- [page/README.md](page/README.md): `/live` page access, states, controls, and side-menu placement
- [page/states-and-controls.md](page/states-and-controls.md): viewer states, video containment, and admin control surface
- [page/admin-workflow.md](page/admin-workflow.md): broadcast start, stop, and runtime override rules
- [page/lifecycle.md](page/lifecycle.md): admin leave behavior, cleanup, and stream-ending rules

### Capture
- [capture/README.md](capture/README.md): capture sources, quality presets, and microphone defaults
- [capture/sources.md](capture/sources.md): screen vs camera, device picker, and permission rules
- [capture/quality-and-constraints.md](capture/quality-and-constraints.md): height, frame-rate presets, and browser best-effort constraints
- [capture/runtime-changes.md](capture/runtime-changes.md): source switching, constraint application, and renegotiation

### Signaling
- [signaling/README.md](signaling/README.md): WebSocket signaling, message shapes, and ICE server contract
- [signaling/messages.md](signaling/messages.md): message types, forwarding rules, and lifetime rules
- [signaling/ice-servers.md](signaling/ice-servers.md): `Live/ICE_servers_JSON` format, STUN vs TURN, and URL schemes
- [signaling/connectivity.md](signaling/connectivity.md): NAT traversal, external ICE services, and connection fallback order

### Network
- [network/README.md](network/README.md): local network topology and firewall rules
- [network/topology.md](network/topology.md): local compose service map and external edge ownership
- [network/firewall-rules.md](network/firewall-rules.md): local and external port ownership

## Start Here

- User-visible page behavior: [page/states-and-controls.md](page/states-and-controls.md)
- Admin capture controls: [capture/sources.md](capture/sources.md)
- Leave and cleanup behavior: [page/lifecycle.md](page/lifecycle.md)
- Transport contract: [signaling/messages.md](signaling/messages.md)
- Remote connectivity requirements: [signaling/connectivity.md](signaling/connectivity.md)
- Local network map: [network/topology.md](network/topology.md)
