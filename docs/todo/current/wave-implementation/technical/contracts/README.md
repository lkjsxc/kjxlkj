# Technical: Contracts (Iteration 34)

Back: [/docs/todo/current/wave-implementation/technical/README.md](/docs/todo/current/wave-implementation/technical/README.md)

## Scope

Implement and enforce the technical contracts and invariants required by the spec.

## Defining documents (direct, normative)

- Contracts:
  - [/docs/spec/technical/contracts.md](/docs/spec/technical/contracts.md)

## Checklist

- [ ] Turn each contract into explicit testable assertions where possible. — done: 18 contract assertion tests in `contract_assertions.rs` (BufferId newtype, Size fields, Mode default, Position ordering, CursorShape, ExCommand prefix, transitions, snapshots)
- [ ] Ensure contracts are enforced at API boundaries (core/services/render).
  - contract_checks.rs: ContractChecker with viewport_bounded, input_ordering, bus_utilization, buffer_ids, no_plugin_loading, restart_limit checks
- [ ] Record any untestable contracts as limitations with a verification plan.
  - contracts.rs: ContractLevel, Violation, ContractChecker (require/ensure/invariant), in_range, non_empty, valid_buffer_id, within_limit

