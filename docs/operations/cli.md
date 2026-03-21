# CLI Contract

`kjxlkj` exposes agent-focused commands with deterministic output.

## Command Groups

- `serve`: run HTTP server.
- `content list --include-private`: list articles as JSON.
- `content validate`: validate content tree readability.
- `content set-private <slug> --value <true|false>`: set visibility.
- `docs validate-topology`: enforce README TOC topology.
- `quality check-lines`: enforce docs/source line limits.
- `system doctor`: report environment diagnostics in JSON.
- `compose verify`: run `docker compose --profile verify run --rm verify`.

## Output Rules

- JSON outputs are stable for agent parsing.
- Non-zero exit code indicates failure.
- Error keys are concise and deterministic.
