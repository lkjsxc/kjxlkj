# TODO: Modes

Back: [/docs/todo/current/areas/README.md](/docs/todo/current/areas/README.md)

## Normative Sources

- [/docs/spec/modes/README.md](/docs/spec/modes/README.md)
- [/docs/spec/modes/normal.md](/docs/spec/modes/normal.md)
- [/docs/spec/modes/insert/README.md](/docs/spec/modes/insert/README.md)
- [/docs/spec/modes/visual.md](/docs/spec/modes/visual.md)
- [/docs/spec/modes/replace/README.md](/docs/spec/modes/replace/README.md)
- [/docs/spec/modes/command.md](/docs/spec/modes/command.md)
- [/docs/spec/modes/transitions.md](/docs/spec/modes/transitions.md)

## Inventory

- [ ] Extract all mode and transition requirements into requirement matrix.

## Implementation

- [ ] Implement each mode's entry/exit behavior and invariant state transitions.
- [ ] Implement transition edge cases (timeouts, cancels, nested prefixes).
- [ ] Ensure mode behavior remains deterministic under async service load.

## Verification

- [ ] Add/refresh deterministic mode transition and regression tests.
- [ ] Record evidence in conformance and limitations ledgers.
