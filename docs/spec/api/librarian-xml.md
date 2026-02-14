# Librarian XML-Like Protocol

Back: [/docs/spec/api/README.md](/docs/spec/api/README.md)

Normative protocol for LLM interaction used by the autonomous librarian agent.

## Protocol Goals

- Preserve deterministic parse behavior even with small-parameter models.
- Avoid attribute parsing complexity by using tag-only envelopes.
- Keep request and response contracts explicit and machine-validated.
- Keep prompt text externalized in JSON prompt-pack files.

## Version and Mode

- Protocol name is `xml_attrless`.
- Payload MUST be UTF-8 plain text.
- Tags MUST NOT include XML attributes.
- Unknown tags MAY appear, but MUST be ignored by parser unless strict mode is enabled.

## Prompt-Pack Binding

- The request-producing prompt MUST come from
  [/docs/spec/technical/librarian-prompts/manifest.json](/docs/spec/technical/librarian-prompts/manifest.json).
- Stage prompt definitions MUST be loaded from JSON files referenced by the manifest.
- Implementations MUST hash prompt-pack files and record hash per run.

## Request Envelope

Root tag MUST be `<librarian_request>`.

Required child tags:

- `<request_id>` unique request identifier
- `<goal>` high-level structuring objective
- `<scope>` workspace, project, or note scope descriptor
- `<source_bundle>` normalized source documents and metadata
- `<taxonomy>` allowed section and topic vocabulary
- `<constraints>` hard rules (safety, size, style)
- `<output_contract>` expected operation schema

The `<source_bundle>` tag MUST contain one or more `<source>` blocks.

Each `<source>` block MUST contain:

- `<source_id>`
- `<title>`
- `<body_markdown>`
- `<kind>`

## Response Envelope

Root tag MUST be `<librarian_response>`.

Required child tags:

- `<request_id>`
- `<status>` (`ok` or `needs_clarification` or `rejected`)
- `<summary>`
- `<operations>`
- `<warnings>`

`<operations>` MAY contain zero or more `<operation>` blocks.

Each `<operation>` block MUST contain:

- `<operation_id>`
- `<kind>` (`create_note`, `rewrite_note`, `retitle_note`, `relink_note`, `retag_note`, `defer`)
- `<target_note_id>` or `<target_path>`
- `<title>`
- `<body_markdown>`
- `<reason>`
- `<confidence>`

## Minimal Example

```xml
<librarian_request>
<request_id>req_01</request_id>
<goal>Organize incident runbooks by subsystem</goal>
<scope>workspace:acme-core</scope>
<source_bundle>
<source>
<source_id>note_18</source_id>
<title>Outage notes</title>
<body_markdown>...</body_markdown>
<kind>markdown</kind>
</source>
</source_bundle>
<taxonomy>
<topic>platform</topic>
<topic>database</topic>
<topic>network</topic>
</taxonomy>
<constraints>
<strict_mode>true</strict_mode>
<max_operations>12</max_operations>
<allow_delete>false</allow_delete>
</constraints>
<output_contract>xml_attrless</output_contract>
</librarian_request>
```

## Parser and Validation Rules

- Parser MUST normalize line endings to `\n` before tokenization.
- Parser MUST reject malformed nesting with `LIBRARIAN_PROTOCOL_INVALID`.
- Missing required tags MUST fail run as `LIBRARIAN_PARSE_FAILED`.
- `<confidence>` MUST parse as decimal in `[0.0, 1.0]`.
- Operations exceeding configured `max_operations` MUST be rejected.

## Retry and Repair Rules

- On parse failure, server MUST run at most two repair retries.
- Retry prompt MUST include only validation errors and the original response.
- If retries fail, run status MUST become `Failed` with deterministic error code.
- Failed parses MUST preserve raw model text for audit.

## Small-Model Compatibility Rules

- Prompts SHOULD avoid deep nesting beyond four levels.
- Prompt vocabulary SHOULD prefer short imperative sentences.
- Output contract MUST ask for one operation per `<operation>` block.
- Server MUST support strict fallback mode where only `create_note` and
  `rewrite_note` operations are accepted.

## Related

- Prompt pack: [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md)
- Automation domain: [/docs/spec/domain/automation.md](/docs/spec/domain/automation.md)
- Payload types: [types.md](types.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
