# Agent Prompt JSON Schema

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Defines the canonical JSON schema for `kjxlkj-agent` prompt configuration.

## File Location

- Primary file: `data/agent-prompt.json`

## Required Shape

```json
{
  "agent_name": "kjxlkj-agent",
  "version": "2026-02-15",
  "default_mode": "yolo",
  "segments": [
    {
      "condition": "default",
      "prompt": "..."
    }
  ]
}
```

## Field Rules

| Field | Requirement |
|---|---|
| `agent_name` | MUST equal `kjxlkj-agent` |
| `version` | semantic or date-based version string |
| `default_mode` | `reviewed` or `yolo` |
| `segments` | non-empty ordered list |

Segment rules:

- `condition` MUST be `default` or a known state label.
- `prompt` MUST be non-empty UTF-8 string.
- Segments are applied in array order.

## Validation Rules

- Invalid JSON MUST hard-fail startup.
- Unknown keys MAY be ignored but SHOULD be logged once.
- Missing required fields MUST fail with `PROMPT_SCHEMA_INVALID`.

## Related

- Agent technical contract: [librarian-agent.md](librarian-agent.md)
- Configuration: [/docs/spec/architecture/configuration.md](/docs/spec/architecture/configuration.md)
