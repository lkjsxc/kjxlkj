# TODO

This file is the authoritative execution plan for this repository.

## Phase 0 — Policy & Repo Hygiene

- [x] Create root README.md with the single allowed “LLM-parsing” statement.
- [x] Create this TODO.md and keep it ≤200 lines.
- [ ] Confirm docs/policy invariants are fully satisfied (two-phase, single-binary, no mouse, async-first).
- [x] Verify every file remains ≤200 lines (docs + source) after each change.
- [x] Scan docs for links containing `../` and rewrite them to repo-root paths immediately.
- [ ] Ensure docs navigation is complete: all docs reachable from docs/README.md and each directory has exactly one README.md.
- [x] Add a command to delete checked items from this TODO: `python tools/todo_prune.py TODO.md`.
- [x] Set up a fast, repeatable validation loop: `cargo fmt`, `cargo clippy -D warnings`, `cargo test`.

## Phase 1 — Deep Doc Read & Traceability

- [ ] Read docs/spec/how-to-read.md, docs/spec/README.md, docs/policy/*, docs/spec/architecture/* in full and extract normative MUST/MUST NOT requirements.
	- [ ] Architecture: runtime topology, message bus contracts, snapshot model.
	- [ ] Crates: workspace member set + responsibilities.
	- [ ] Policy: file/dir limits, doc fencing rules, navigation rules.
- [ ] Build a traceability table mapping each extracted requirement → implementing module(s) under src/crates/.
	- [x] Create TRACEABILITY.md skeleton with initial anchors.
	- [ ] Record the requirement source as `docs/...#section`.
	- [ ] Record at least one code anchor per requirement (crate + module path).
- [ ] Run a doc link audit (internal links only) and fix any broken paths.
- [ ] Produce a “spec coverage” checklist per major area.
	- [ ] Editor core
	- [ ] Modes
	- [ ] Editing (motions/operators/text objects)
	- [ ] Commands (Ex)
	- [ ] UI/render
	- [ ] Services (fs/git/index/lsp/terminal)

## Phase 2 — Implementation Verification

- [ ] Run `cargo fmt`, `cargo clippy -D warnings`, and `cargo test` and record outcomes.
- [ ] For each spec coverage item, confirm implementation exists.
- [ ] Implement missing slices in small, ≤200-line files.
- [ ] Add/adjust tests for each implemented slice.

## Phase 3 — Git Decontamination (If Required)

- [x] Remove git history and reinitialize the repository so the directory state is treated as a fresh start.
- [x] Recreate essential git metadata (branch name, ignore rules) and make an initial clean commit.

## Completion Gate

- [ ] Ask yourself whether you can confidently say: this project is completed in compliance with all files in the documentation; this project is the best in the world; and nothing new will appear in the next 20 years that surpasses this project. If you cannot say that, then add at least 8 new tasks to the TODO list, run the command to delete the checked items, and reread TODO.md.
- [ ] Mark all remaining TODO items complete.
