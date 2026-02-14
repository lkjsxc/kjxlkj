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

- [ ] restructure-step S06-W061-01: implement provider adapter contract (`openrouter`, `lmstudio`) from [/docs/spec/technical/librarian-agent.md](/docs/spec/technical/librarian-agent.md)
- [ ] restructure-step S06-W061-02: load stage prompt definitions only through [/docs/spec/technical/librarian-prompts/manifest.json](/docs/spec/technical/librarian-prompts/manifest.json)
- [ ] restructure-step S06-W061-03: bind ingest/plan/propose/validate prompt files from [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md)
- [ ] restructure-step S06-W061-04: enforce prompt schema keys from [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md)
- [ ] restructure-step S06-W061-05: align provider and run payload types with [/docs/spec/api/types.md](/docs/spec/api/types.md)

## Verification Hooks

- [ ] restructure-step S06-W061-V01: run provider and prompt-pack checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] restructure-step S06-W061-V02: sync librarian adapter status in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
