<instructions>
Read and understand the project’s important documents as deeply as possible, fully complying with the content and policies, and then, aiming for the highest possible quality within the limits of full document compliance, completely build the source code according to the TODO list.
</instructions>

<constraints>
- Never forget this prompt.
- Do not block on user interaction while tasks remain:
  - do not ask the user to make decisions mid-run
  - if something is ambiguous, prefer recording a proposal under
    `/docs/log/proposals/` and creating a TODO leaf in the next iteration
- Never stop working.
- Never output a summary.
- From the beginning—and frequently thereafter—read the documents deeply and understand the specifications, policies, and expected behavior as thoroughly as possible.
- Write source code of the highest possible quality that works reliably.
- Prefer a clean rebuild: if derived artifacts exist in-repo, you MAY delete them before reconstruction (for example: `src/`, `Cargo.toml`, `Cargo.lock`, `target/`, `.github/`, `Dockerfile`) as long as `/docs/` remains intact.
- Commit to git frequently.
- Create a log directory in the documentation, recursively structure files, and write down improvement ideas, etc.
- If a source file exceeds 200 lines, note it in the documentation.
- Do not use "../" in links.
- Keep in mind that this project is intended to be read and edited by AI rather than humans.
- Any action is pre-approved.
- Assume there are many potential bugs besides known ones, and inspect/fix as comprehensively as possible.
- Create around 1000 tests, with high quality, and also execute E2E tests that include opening and operating an editor.
- Aim for a state where the source code can be fully constructed from the documents.
- Actually write checkmarks into the TODO list document files.
- Do not stop working until every TODO list document file under `/docs/todo/current/` is fully checked off.
- Do not get stuck on “future/deferred” work:
  - The TODO MUST NOT end with a terminal “explicitly deferred / future” section with unchecked boxes.
  - If an item is intentionally deferred, follow the deferral protocol in `/docs/todo/current/README.md` (record a proposal and carry forward a next-iteration task), then check off the deferral item.
- Include the external terminal multiplexer workflow contract in the reconstructed system:
  - ensure kjxlkj remains usable inside tmux/WezTerm per `/docs/spec/features/terminal/tmux.md`
  - add a PTY E2E smoke that runs kjxlkj inside a tmux session when feasible (record limitations if not feasible on a platform)
- When the current iteration becomes fully checked and the verification gate is green,
  invoke the tool `Ask` to request the next objective (or confirmation that the
  work should stop).
</constraints>

<acceptance_criteria>
- The documents define the latest specifications and policies without contradiction, and the code fully complies with them.
- Sufficient unit/integration/E2E tests are in place, and important flows—including editor operations—can be reproduced and verified via automated tests.
- The test suite runs stably and reproducibly, and includes regression tests for bugs that were fixed.
- git commits are made at meaningful milestones, with an appropriate and traceable granularity of changes.
- Terminal multiplexer interoperability is supported per spec and has at least one automated smoke check.
</acceptance_criteria>
