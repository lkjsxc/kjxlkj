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

- [x] Extract all normative requirements from listed architecture specs into requirement matrix.
- [x] Assign stable IDs and map each ID to implementation and test evidence paths.
- [x] Identify parallel agent/runtime implementations and map each to canonical architecture requirements.

## Implementation

- [x] Implement startup/shutdown sequence exactly as specified.
- [x] Implement runtime task topology and channel topology exactly as specified.
- [x] Consolidate parallel agent/runtime paths into one canonical input->core->render/service flow.
- [x] Ensure single-writer core ownership and snapshot-only rendering path.
- [x] Enforce built-in integrations policy (no external plugin loading).

## Verification

- [x] Add/refresh deterministic tests for startup, runtime orchestration, and signal handling.
- [x] Verify only one canonical core dispatch owner exists in runtime behavior.
- [x] Verify architecture claims in reference ledgers with evidence links.
