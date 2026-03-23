# Purpose Contract

## Goal

`kjxlkj` is a deterministic record service designed for LLM-operated workflows.

## Product Intent

- Serve structured records over HTTP JSON endpoints.
- Keep read paths public and write paths token-protected.
- Preserve records durably on local filesystem storage.
- Keep outputs deterministic for automation and replay.

## Non-Goals

- No browser-first UX requirements.
- No backward compatibility guarantees.
- No mutable behavior outside documented contracts.
