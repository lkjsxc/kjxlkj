# Optional Staged Prompt Pack

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

This directory contains optional staged prompt assets.

## Canonical Rule

- The mandatory runtime prompt source is `data/agent-prompt.json`.
- Files in this directory MAY be used to generate/compose that runtime prompt.

## Files

| File | Purpose |
|---|---|
| [manifest.json](manifest.json) | stage file mapping |
| [stage-ingest.json](stage-ingest.json) | ingest prompt segment |
| [stage-plan.json](stage-plan.json) | planning prompt segment |
| [stage-propose.json](stage-propose.json) | propose prompt segment |
| [stage-validate-repair.json](stage-validate-repair.json) | repair prompt segment |

## Related

- Agent prompt schema: [../agent-prompt-json.md](../agent-prompt-json.md)
- Agent contract: [../librarian-agent.md](../librarian-agent.md)
