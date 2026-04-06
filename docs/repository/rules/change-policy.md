# Change Policy

## Sequence

1. Update contracts in `docs/`.
2. Update code to satisfy contracts.
3. Run mandatory Rust and compose gates.
4. Commit the verified batch.

## Blocking Rule

If docs and code diverge, code changes are incomplete.

## Navigation Rule

- Update parent `README.md` files when children are added, removed, or moved.
- Remove stale locations instead of keeping compatibility copies.
