# Librarian Prompt Pack (JSON)

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Canonical JSON-only prompt configuration for the AI librarian cycle.

## Contract

- Every LLM prompt in the librarian cycle MUST be defined in JSON files here.
- Stage files MUST be referenced by `manifest.json` only.
- Runtime implementations MUST load prompts from these JSON definitions.

## Files

| File | Purpose |
|---|---|
| [manifest.json](manifest.json) | prompt pack metadata and stage-to-file mapping |
| [stage-ingest.json](stage-ingest.json) | source normalization prompt template |
| [stage-plan.json](stage-plan.json) | structuring plan prompt template |
| [stage-propose.json](stage-propose.json) | operation proposal prompt template |
| [stage-validate-repair.json](stage-validate-repair.json) | bounded repair/retry prompt template |

## JSON Schema Expectations

Each stage JSON file MUST include:

- `stage`
- `model_profile`
- `temperature`
- `max_tokens`
- `system_prompt`
- `user_prompt_template`
- `input_bindings`
- `output_contract`

## Related

- Technical contract: [../librarian-agent.md](../librarian-agent.md)
- API protocol: [/docs/spec/api/librarian-xml.md](/docs/spec/api/librarian-xml.md)
