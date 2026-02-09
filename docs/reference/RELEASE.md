# Release Process

Back: [/docs/reference/README.md](/docs/reference/README.md)

Release is valid only for a reconstructed, verified state.

## Preconditions

1. CI gate is green for reconstructed profile.
2. `CONFORMANCE` claims are evidence-backed.
3. `LIMITATIONS` reflects all remaining user-visible gaps.
4. Target scope for release is explicit.

## Release Steps

1. Freeze docs and implementation together.
2. Run verification gate and capture evidence.
3. Create release commit and tag.
4. Publish artifacts.
5. Record release evidence links.

## Post-Release

- Start the next reconstruction wave in `/docs/todo/current/`.
- Update conformance and limitations for the new baseline.
