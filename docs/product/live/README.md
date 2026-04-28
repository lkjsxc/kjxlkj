# Live Broadcast

Use this subtree for the `/live` public broadcast page, its in-app WebRTC relay contract, and remote connectivity requirements.

## Read This Section When

- You need the intended admin screen, camera, quality, and microphone workflow.
- You need the public viewer behavior for the live page.
- You need the WebSocket signaling, ICE-server, and in-app relay contract.
- You need the production network shape for remote viewers.

## Child Index

### Page
- [page/README.md](page/README.md): `/live` page access, states, controls, and side-menu placement
- [page/states-and-controls.md](page/states-and-controls.md): viewer states, video containment, and admin control surface
- [page/lifecycle.md](page/lifecycle.md): admin leave behavior, cleanup, and stream-ending rules

### Capture
- [capture/README.md](capture/README.md): capture sources, quality presets, microphone defaults, and runtime changes
- [capture/sources.md](capture/sources.md): screen, camera, device picker, and browser permission rules
- [capture/quality-and-audio.md](capture/quality-and-audio.md): quality presets, microphone behavior, and runtime changes

### Signaling
- [signaling/README.md](signaling/README.md): WebSocket signaling, messages, app relay ICE, and failure behavior
- [signaling/messages.md](signaling/messages.md): message shapes, forwarding, and lifetime rules
- [signaling/ice-servers.md](signaling/ice-servers.md): app relay ICE environment and address rules
- [signaling/connectivity.md](signaling/connectivity.md): NAT traversal and failure states

### Network
- [network/README.md](network/README.md): local compose and production edge ownership
- [network/topology.md](network/topology.md): local service map and production edge flow
- [network/firewall-rules.md](network/firewall-rules.md): required HTTP, WebSocket, and app ICE ports

## Start Here

- User-visible page behavior: [page/states-and-controls.md](page/states-and-controls.md)
- Admin capture controls: [capture/sources.md](capture/sources.md)
- Leave and cleanup behavior: [page/lifecycle.md](page/lifecycle.md)
- Transport contract: [signaling/messages.md](signaling/messages.md)
- Remote connectivity: [signaling/connectivity.md](signaling/connectivity.md)
- Production edge setup: [network/topology.md](network/topology.md)
