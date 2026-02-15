# Wave 061: Librarian Provider Adapters and JSON Prompt Loading

Back: [/docs/todo/waves/stage-06-rest-api/README.md](/docs/todo/waves/stage-06-rest-api/README.md)

## Relevant Documents

- [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)
- [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md)
- [/docs/spec/technical/librarian-prompts/manifest.json](/docs/spec/technical/librarian-prompts/manifest.json)
- [/docs/spec/technical/librarian-prompts/stage-ingest.json](/docs/spec/technical/librarian-prompts/stage-ingest.json)
- [/docs/spec/technical/librarian-prompts/stage-plan.json](/docs/spec/technical/librarian-prompts/stage-plan.json)
- [/docs/spec/technical/librarian-prompts/stage-propose.json](/docs/spec/technical/librarian-prompts/stage-propose.json)
- [/docs/spec/technical/librarian-prompts/stage-validate-repair.json](/docs/spec/technical/librarian-prompts/stage-validate-repair.json)
- [/docs/spec/api/types.md](/docs/spec/api/types.md)

## Restructure Steps

- [x] restructure-step S06-W061-01: implement provider adapter contract (`openrouter`, `lmstudio`) from [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md) [doc-link](/docs/spec/technical/librarian-agent.md)
- [x] restructure-step S06-W061-02: load stage prompt definitions only through [/docs/spec/technical/librarian-prompts/manifest.json](/docs/spec/technical/librarian-prompts/manifest.json) [doc-link](/docs/spec/technical/librarian-prompts/manifest.json)
- [x] restructure-step S06-W061-03: bind ingest/plan/propose/validate prompt files from [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md) [doc-link](/docs/spec/technical/librarian-prompts/README.md)
- [x] restructure-step S06-W061-04: enforce prompt schema keys from [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md) [doc-link](/docs/spec/technical/librarian-prompts/README.md)
- [x] restructure-step S06-W061-05: align provider and run payload types with [/docs/spec/api/types.md](/docs/spec/api/types.md) [doc-link](/docs/spec/api/types.md)

## Verification Hooks

- [x] restructure-step S06-W061-V01: run provider and prompt-pack checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [x] restructure-step S06-W061-V02: sync librarian adapter status in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) [doc-link](/docs/reference/CONFORMANCE.md)
