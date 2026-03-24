# Constraints and Invariants

## Hard Constraints

1. Every docs directory contains exactly one `README.md` TOC.
2. Every docs directory contains multiple child entries.
3. Every docs file is 300 lines or fewer.
4. Every source file is 200 lines or fewer.
5. Canonical definitions are singular and linked from other locations.

## Execution Invariants

- Stages execute in ordered sequence only.
- Waves execute in listed order within each stage.
- Any failed gate blocks stage closure.
- Evidence must state pass, fail, or blocked explicitly.
- Blocked states require an explicit cause and recovery plan.

## Verification Invariants

- Structure audit is mandatory before final acceptance.
- Link integrity audit is mandatory before final acceptance.
- Compose verification is required when compose assets exist.
