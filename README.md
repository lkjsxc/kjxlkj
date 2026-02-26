# kjxlkj

All-in-docs workspace-suite platform for notes, search, and `kjxlkj-agent` automation.

---

## Current Mode

This repository is in **documentation-first reconstruction mode**.

- Canonical implementation contract lives under `docs/`
- Legacy runtime state has been captured in reference ledgers
- Rebuild execution is governed by linked TODO waves
- Release is blocked until acceptance and evidence gates are closed

Start here:

1. [docs/README.md](docs/README.md)
2. [docs/todo/README.md](docs/todo/README.md)
3. [docs/reference/CONFORMANCE.md](docs/reference/CONFORMANCE.md)
4. [docs/reference/DRIFT_MATRIX.md](docs/reference/DRIFT_MATRIX.md)

---

## Authority Order

When documents disagree, precedence is:

1. `docs/policy/`
2. `docs/spec/`
3. `docs/reference/`
4. `docs/todo/`
5. `docs/guides/`

---

## Reconstruction Rules

- Every TODO checkbox must link to governing docs
- Every checkbox closure requires explicit evidence linkage
- Acceptance IDs and suite categories come from `docs/spec/technical/testing.md`
- Matrix closure is mandatory in:
  - `docs/reference/TODO_TRACE_MATRIX.md`
  - `docs/reference/TEST_MATRIX.md`
  - `docs/reference/EVIDENCE_INDEX.md`

---

## Documentation Map

- Policy: `docs/policy/`
- Overview: `docs/overview/`
- Spec: `docs/spec/`
- Reference: `docs/reference/`
- Guides: `docs/guides/`
- TODO program: `docs/todo/`

---

## License

MIT License. See [LICENSE](LICENSE).
