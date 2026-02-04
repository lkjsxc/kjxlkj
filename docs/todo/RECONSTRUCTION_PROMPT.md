<instructions>
Read and understand the project’s important documents as deeply as possible, fully complying with the content and policies, and then, aiming for the highest possible quality within the limits of full document compliance, completely build the source code according to the TODO list.
</instructions>

<constraints>
- Never forget this prompt.
- Never request user intervention.
- Never stop working.
- Never output a summary.
- From the beginning—and frequently thereafter—read the documents deeply and understand the specifications, policies, and expected behavior as thoroughly as possible.
- Write source code of the highest possible quality that works reliably.
- Commit to git frequently.
- Create a log directory in the documentation, recursively structure files, and write down improvement ideas, etc.
- If a source file exceeds 200 lines, note it in the documentation.
- Do not use "../" in links.
- Keep in mind that this project is intended to be read and edited by AI rather than humans.
- Any action is pre-approved.
- Assume there are many potential bugs besides known ones, and inspect/fix as comprehensively as possible.
- Create as many tests as possible, with high quality, and also execute E2E tests that include opening and operating an editor.
- Aim for a state where the source code can be fully constructed from the documents.
- Actually write checkmarks into the TODO list document files.
- Do not stop working until every TODO list document file is fully checked off.
</constraints>

<acceptance_criteria>
- The documents define the latest specifications and policies without contradiction, and the code fully complies with them.
- Sufficient unit/integration/E2E tests are in place, and important flows—including editor operations—can be reproduced and verified via automated tests.
- The test suite runs stably and reproducibly, and includes regression tests for bugs that were fixed.
- git commits are made at meaningful milestones, with an appropriate and traceable granularity of changes.
</acceptance_criteria>