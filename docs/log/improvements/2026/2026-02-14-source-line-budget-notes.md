# 2026-02-14 Source Line Budget Notes

Back: [/docs/log/improvements/2026/README.md](/docs/log/improvements/2026/README.md)

## Improvement Ideas

- split `handlers/automation.rs` by route family (`rules`, `runs`, `review`,
  `providers`, `parser`) and keep each module below 200 lines
- split `handlers/notes.rs` into lifecycle/history/metadata/search handlers
- split websocket handlers into connection/session/replay/ack modules
- move long test fixtures/helpers into shared test support modules
- add CI check that reports runtime source files over 200 lines as warnings

## Immediate Next Slice

Create a deterministic module-extraction plan for server handlers and db repos
without changing external API behavior.
