# Librarian Agent Guide

Back: [/docs/guides/README.md](/docs/guides/README.md)

Practical setup and usage guide for autonomous documentation structuring.

## Scope

This guide covers configuring librarian automation rules with:

- OpenRouter-hosted models
- LM Studio local models

## Rule Payload Skeleton

Create rule via `POST /api/automation/rules`.

```json
{
  "trigger": "workspace_event.note_changed",
  "condition_json": { "workspace_id": "ws_01" },
  "action_json": {
    "kind": "librarian_structure",
    "protocol": "xml_attrless",
    "provider": {
      "provider_kind": "openrouter",
      "base_url": "https://openrouter.ai/api/v1",
      "model": "openrouter/auto",
      "timeout_ms": 20000,
      "max_tokens": 1800,
      "temperature": 0.2,
      "fallback_models": ["openrouter/auto"]
    },
    "plan": {
      "goal": "Organize docs by architecture domain",
      "scope": "workspace:core-docs",
      "style_profile": "technical-rfc",
      "strict_mode": true,
      "max_operations": 12
    }
  },
  "enabled": true
}
```

## OpenRouter Setup

- Configure provider kind `openrouter`.
- Use HTTPS `base_url` and runtime-managed API key secret.
- Keep deterministic settings (`temperature` low, explicit token limit).

## LM Studio Setup

- Configure provider kind `lmstudio`.
- Default local `base_url` is `http://127.0.0.1:1234/v1`.
- Choose a model that can follow strict tag-only output.
- Keep `strict_mode` enabled for smaller models.

## XML-Like Protocol Notes

- Librarian interaction uses an attribute-less XML-like format.
- Do not include attributes in tags.
- Keep responses as repeated `<operation>` blocks with explicit fields.
- Use protocol reference:
  [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md).

## Operational Tips

- Start with smaller source bundles to calibrate model quality.
- Review run reports from `GET /api/automation/runs/{id}`.
- Enable manual review mode when introducing new taxonomy rules.
- Keep fallback model list short and deterministic.

## Related

- API guide: [API.md](API.md)
- Automation domain: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- Technical contract:
  [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)
