# TODO: Architecture

Back: [/docs/todo/current/areas/README.md](/docs/todo/current/areas/README.md)

## Normative Sources

- [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- [/docs/spec/architecture/startup.md](/docs/spec/architecture/startup.md)
- [/docs/spec/architecture/input-decoding.md](/docs/spec/architecture/input-decoding.md)
- [/docs/spec/architecture/render-pipeline.md](/docs/spec/architecture/render-pipeline.md)
- [/docs/spec/architecture/plugins.md](/docs/spec/architecture/plugins.md)
- [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)

## Inventory

- [ ] Extract all normative requirements from listed architecture specs into requirement matrix.
- [ ] Assign stable IDs and map each ID to implementation and test evidence paths.
- [ ] Identify parallel agent/runtime implementations and map each to canonical architecture requirements.

## Implementation

- [ ] Implement startup/shutdown sequence exactly as specified.
- [ ] Implement runtime task topology and channel topology exactly as specified.
- [ ] Consolidate parallel agent/runtime paths into one canonical input->core->render/service flow.
- [ ] Ensure single-writer core ownership and snapshot-only rendering path.
- [ ] Enforce built-in integrations policy (no external plugin loading).

## Verification

- [ ] Add/refresh deterministic tests for startup, runtime orchestration, and signal handling.
- [ ] Verify only one canonical core dispatch owner exists in runtime behavior.
- [ ] Verify architecture claims in reference ledgers with evidence links.
