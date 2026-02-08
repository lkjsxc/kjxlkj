# TODO: Verification

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Verification gates

Per [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md):

### Gate 0: Baseline audit

- [ ] Run verification commands from [/docs/reference/CI.md](/docs/reference/CI.md)
- [ ] Create mismatch matrix (M1-M5 classification)
- [ ] Record audit under `/docs/log/reconstruction/audits/`

### Gate 1: Slice definition

- [ ] Choose coherent slice from this TODO
- [ ] Define acceptance criteria with spec references
- [ ] Define required tests per testing contract

### Gate 2: Implement

- [ ] Wire behavior through real user-reachable paths
- [ ] Verify each feature callable from binary `main` through real input
- [ ] Keep source files under 200 lines per [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md)

### Gate 3: Verify and synchronize

- [ ] Run touched tests first, then full suite
- [ ] Update [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [ ] Update [/docs/reference/CONFORMANCE_EDITING_OPERATORS.md](/docs/reference/CONFORMANCE_EDITING_OPERATORS.md)
- [ ] Update [/docs/reference/CONFORMANCE_EDITING_FEATURES.md](/docs/reference/CONFORMANCE_EDITING_FEATURES.md)
- [ ] Update [/docs/reference/CONFORMANCE_MODES.md](/docs/reference/CONFORMANCE_MODES.md)
- [ ] Update [/docs/reference/CONFORMANCE_KEYS_INPUT.md](/docs/reference/CONFORMANCE_KEYS_INPUT.md)
- [ ] Update [/docs/reference/CONFORMANCE_KEYS_SYSTEMS.md](/docs/reference/CONFORMANCE_KEYS_SYSTEMS.md)
- [ ] Update [/docs/reference/CONFORMANCE_KEYS_INFRA.md](/docs/reference/CONFORMANCE_KEYS_INFRA.md)
- [ ] Update [/docs/reference/CONFORMANCE_COMMANDS.md](/docs/reference/CONFORMANCE_COMMANDS.md)
- [ ] Update [/docs/reference/CONFORMANCE_COMMANDS_TYPES.md](/docs/reference/CONFORMANCE_COMMANDS_TYPES.md)
- [ ] Update [/docs/reference/CONFORMANCE_TESTING.md](/docs/reference/CONFORMANCE_TESTING.md)
- [ ] Update [/docs/reference/CONFORMANCE_TESTING_INFRA.md](/docs/reference/CONFORMANCE_TESTING_INFRA.md)
- [ ] Update [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [ ] Check TODO items only after evidence is green

## Acceptance criteria

All must be true:

- [ ] Behavior matches selected target and linked specs
- [ ] Conformance/limitations match observed behavior
- [ ] Required deterministic tests are green at all layers
- [ ] TODO checkboxes reflect proven completion
- [ ] Terminal emulator spawns real PTY processes
- [ ] Session save/load produces/reads valid JSON per schema
- [ ] CJK cursor never occupies half-cell position
- [ ] Long lines wrap correctly with CJK boundary padding
- [ ] Terminal multiplexer contract verified or limited with closure plan
- [ ] Code volume meets minimums per [/docs/log/proposals/anti-mvp-measures.md](/docs/log/proposals/anti-mvp-measures.md)

## Reference documents

- [/docs/reference/README.md](/docs/reference/README.md)
- [/docs/reference/CI.md](/docs/reference/CI.md)
- [/docs/reference/COMPARISON.md](/docs/reference/COMPARISON.md)
- [/docs/reference/PLUGIN_MAPPING.md](/docs/reference/PLUGIN_MAPPING.md)
- [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)
