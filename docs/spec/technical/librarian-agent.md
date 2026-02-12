# Librarian Agent Technical Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Technical requirements for autonomous information structuring with LLM providers.

## Scope

- The librarian agent autonomously transforms raw notes/documents into structured,
  cross-linked documentation artifacts.
- The feature operates through automation rules and run state machine semantics.
- Provider modes MUST support OpenRouter and LM Studio through OpenAI-compatible APIs.

## Pipeline Stages

| Stage | Requirement |
|---|---|
| `ingest` | normalize markdown/settings/media-note metadata into source bundle |
| `plan` | generate taxonomy-aware structuring plan using deterministic prompt contract |
| `propose` | emit candidate operations in `xml_attrless_v1` |
| `validate` | parse and enforce scope/safety/size constraints |
| `apply` | execute accepted operations with optimistic version checks |
| `index` | refresh search/backlink projections and emit automation events |

## Provider Adapter Contract

| Field | Requirement |
|---|---|
| `provider_kind` | `openrouter` or `lmstudio` |
| `base_url` | explicit endpoint URL |
| `model` | model identifier string |
| `timeout_ms` | bounded timeout per request |
| `max_tokens` | hard upper bound |
| `temperature` | deterministic low-variance default |
| `fallback_models` | ordered fallback model IDs |

## Provider-Specific Rules

- `openrouter` mode MUST send authenticated HTTPS requests with bearer token.
- `lmstudio` mode MUST support local endpoints (default `http://127.0.0.1:1234/v1`).
- Adapter MUST expose deterministic failure categories:
  `auth_failed`, `rate_limited`, `timeout`, `unreachable`, `invalid_payload`.
- Failed primary model calls MAY fallback in configured order.

## Prompt and Protocol Rules

- Request/response format MUST follow
  [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md).
- Prompt templates MUST avoid XML attributes.
- Parser MUST reject malformed tags with stable error code.
- Strict mode MUST allow operation-kind allowlists for smaller models.

## Safety and Governance Rules

- Default policy MUST prevent deletion operations.
- Cross-workspace writes MUST be rejected.
- Runs MUST be idempotent per `(rule_id, triggering_event_id)`.
- All accepted operations MUST be persisted with audit metadata.
- Manual review mode MAY require explicit human approval before apply stage.

## Determinism and Reproducibility

- Prompt payload hash MUST be recorded per run.
- Parser version MUST be recorded per run.
- Run reports MUST include rejected operation reasons.
- Validation retries MUST be bounded and deterministic.

## Related

- Automation domain: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- API protocol: [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- Testing: [testing.md](testing.md)
- Operations: [operations.md](operations.md)
