# TODO: Verification

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Verification gates

Per [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md):

### Gate 0: Baseline audit

- [x] Run verification commands from [/docs/reference/CI.md](/docs/reference/CI.md)
- [x] Create mismatch matrix (M1-M5 classification)
- [x] Record audit under `/docs/log/reconstruction/audits/`

### Gate 1: Slice definition

- [x] Choose coherent slice from this TODO
- [x] Define acceptance criteria with spec references
- [x] Define required tests per testing contract

### Gate 2: Implement

- [x] Wire behavior through real user-reachable paths
- [x] Verify each feature callable from binary `main` through real input
- [x] Keep source files under 200 lines per [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)

### Gate 3: Verify and synchronize

- [x] Run touched tests first, then full suite
- [x] Update [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [x] Update [/docs/reference/CONFORMANCE_EDITING_OPERATORS.md](/docs/reference/CONFORMANCE_EDITING_OPERATORS.md)
- [x] Update [/docs/reference/CONFORMANCE_EDITING_FEATURES.md](/docs/reference/CONFORMANCE_EDITING_FEATURES.md)
- [x] Update [/docs/reference/CONFORMANCE_MODES.md](/docs/reference/CONFORMANCE_MODES.md)
- [x] Update [/docs/reference/CONFORMANCE_KEYS_INPUT.md](/docs/reference/CONFORMANCE_KEYS_INPUT.md)
- [x] Update [/docs/reference/CONFORMANCE_KEYS_SYSTEMS.md](/docs/reference/CONFORMANCE_KEYS_SYSTEMS.md)
- [x] Update [/docs/reference/CONFORMANCE_KEYS_INFRA.md](/docs/reference/CONFORMANCE_KEYS_INFRA.md)
- [x] Update [/docs/reference/CONFORMANCE_COMMANDS.md](/docs/reference/CONFORMANCE_COMMANDS.md)
- [x] Update [/docs/reference/CONFORMANCE_COMMANDS_TYPES.md](/docs/reference/CONFORMANCE_COMMANDS_TYPES.md)
- [x] Update [/docs/reference/CONFORMANCE_TESTING.md](/docs/reference/CONFORMANCE_TESTING.md)
- [x] Update [/docs/reference/CONFORMANCE_TESTING_INFRA.md](/docs/reference/CONFORMANCE_TESTING_INFRA.md)
- [x] Update [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [x] Check TODO items only after evidence is green

## Acceptance criteria

All must be true:

- [x] Behavior matches selected target and linked specs
- [x] Conformance/limitations match observed behavior
- [x] Required deterministic tests are green at all layers
- [x] TODO checkboxes reflect proven completion
- [x] Terminal emulator spawns real PTY processes
- [x] Session save/load produces/reads valid JSON per schema
- [x] CJK cursor never occupies half-cell position
- [x] Long lines wrap correctly with CJK boundary padding
- [x] Terminal multiplexer contract verified or limited with closure plan
- [x] Code volume meets minimums per [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md)

## Reference documents

- [/docs/reference/README.md](/docs/reference/README.md)
- [/docs/reference/CI.md](/docs/reference/CI.md)
- [/docs/reference/COMPARISON.md](/docs/reference/COMPARISON.md)
- [/docs/reference/PLUGIN_MAPPING.md](/docs/reference/PLUGIN_MAPPING.md)
- [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
