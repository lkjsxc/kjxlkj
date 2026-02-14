# Librarian Agent Technical Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Technical requirements for autonomous information structuring with LLM providers.

## Scope

- The librarian agent autonomously transforms raw notes/documents into structured,
  cross-linked documentation artifacts.
- The feature operates through automation rules and run state machine semantics.
- Provider modes MUST support OpenRouter and LM Studio through OpenAI-compatible APIs.
- Every prompt sent to an LLM in the agent cycle MUST be loaded from JSON files.

## Pipeline Stages

| Stage | Requirement |
|---|---|
| `ingest` | normalize markdown/settings/media-note metadata into source bundle |
| `plan` | generate taxonomy-aware structuring plan using deterministic prompt contract |
| `propose` | emit candidate operations in `xml_attrless` |
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

## Prompt Configuration Rules (JSON-Only)

- Prompt files MUST be JSON and MUST be the only prompt source.
- Runtime MUST NOT embed prompt literals in source code.
- Prompt pack root MUST be `docs/spec/technical/librarian-prompts/` in canonical
  docs, and MAY be copied to runtime config path during reconstruction.
- Prompt stage definitions MUST be referenced through
  `docs/spec/technical/librarian-prompts/manifest.json`.
- A prompt stage is valid only when all required keys exist in the stage JSON file.
- Missing/malformed stage JSON MUST fail run start with deterministic config error.

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
- Prompt pack version + stage file hashes MUST be recorded per run.
- Parser version MUST be recorded per run.
- Run reports MUST include rejected operation reasons.
- Validation retries MUST be bounded and deterministic.

## Related

- Prompt pack: [librarian-prompts/README.md](librarian-prompts/README.md)
- Automation domain: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- API protocol: [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
- Testing: [testing.md](testing.md)
- Operations: [operations.md](operations.md)
