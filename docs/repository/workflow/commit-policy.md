# Commit Policy Contract

## Frequency

- Commit after each coherent verified batch.
- Prefer smaller batches with a single clear purpose.
- Land docs-only batches before dependent code batches.
- Do not accumulate unrelated verified work into one oversized commit.

## Preconditions

- The changed batch passes the relevant compose and Rust gates.
- Commit message summarizes changed contracts and behavior.
- The docs batch lands before dependent code batches.

## Typical Batch Shapes

- navigation or documentation IA changes
- single contract update plus the code that satisfies it
- verification or tooling updates tied to one workflow change
