# Baseline Evidence

## Purpose

Capture pre-rebuild gate status before structural rewrite.

## Snapshot Fields

- docs topology status
- docs term status
- line-limit status
- cargo test status
- compose verify status

## Update Rule

## Baseline Snapshot

- `cargo run --bin kjxlkj -- docs validate-topology` -> pass (`violations: 0`)
- `cargo run --bin kjxlkj -- docs validate-terms` -> pass (`violations: 0`)
- `cargo run --bin kjxlkj -- quality check-lines` -> pass (`violations: 0`)
- `cargo test` -> pass
- `docker compose --profile verify run --rm verify` -> pass
