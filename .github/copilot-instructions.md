<instructions>
To re-implement everything except the documentation, improve the entire documentation to the best possible state, and then stand by—for example, by unchecking the TODO list items at the end.
</instructions>

<constraints>
- Never ask for user intervention.
- Never stop working.
- Do not change anything other than the documentation.
- For past records, reflect only the necessary parts into the documentation, then delete them completely.
- From the start, and frequently, read the documentation deeply to understand specifications, policies, and expected behavior as thoroughly as possible.
- Write documentation that is as high-quality and reliable as possible.
- Under the log directory, consider specification changes and documentation improvements.
- Commit to git frequently.
- Prepare a log directory within the documentation; recursively structure files and write up improvement proposals and similar notes.
- For any source file that exceeds 200 lines, ensure that the next implementation structures files well so that it does not happen again.
- Among the actually implemented tests, select the desirable ones; also autonomously devise more imaginative, professional-grade end-to-end tests so boundary behavior can be examined.
- Think through test content to the highest possible quality, incorporate it into the documentation, and have implementations be checked/verified incrementally via the TODO list.
- Do not use "../" in links.
- Keep in mind that this project is assumed to be read and edited by AI, not humans.
- Any action is pre-authorized.
- Assume there are many potential bugs in addition to known bugs, and inspect as comprehensively as possible.
- Aim for a state where the full source code can be constructed entirely from the documentation.
- In addition to the TODO list, all bold and substantial changes required for better documentation are permitted to the fullest extent.
- Ensure the TODO list directly links to all documentation files.
- After documentation improvements are done, the user plans to delete everything except the documentation and have Claude Opus 4.5 build it again. Please ensure a complete build will be possible.
</constraints>

<context>
- Currently, starting from a state where nothing exists except the documentation, implementation was requested via GitHub Copilot using Claude Opus 4.5 (said to be the strongest LLM for coding at present), and a notification was received saying it is fully completed.
- Please revise the documentation and add more detail where necessary.
- Also write an overview of the tests that should be written in the documentation, and have the source-code implementation refer to it. Tests should be only as many as necessary: autonomously judge what is needed, and add missing edge cases if any.
- Provide proper checkboxes in the TODO.
- If you judge that a bold, large-scale redesign of the TODO list would make it better, you are allowed to redesign and recreate it as many times as needed. Within the TODO list, verify whether all documentation files are directly linked; if anything is missing, redesign the TODO list.
- Even after implementing all TODOs, the amount of implemented code is only about one-tenth of what was expected. This is likely because it ended up as a minimal implementation (MVP). Address this by increasing TODOs, or by devising measures so that the agent does not take shortcuts while completing TODOs (i.e., so it fully implements functionality).
- The functionality is complete, but it is likely that the app still does not use it in many places. Improve the documentation so that does not happen.
- Japanese behavior is incorrect. Improve the documentation. Full-width characters commonly used in Japanese have a width of 2, but handling around that is not working well. For example, when there is "あいうえお" and you try to move the cursor from "あ" to "い", the cursor moves to the “back half” of "あ", and the cursor display also disappears. The “back half” state behind characters is especially strange, so you need to boldly and drastically change the approach and fix it somehow.
- Document that when a single line becomes too long and overflows the screen, it should wrap to the next line. (Currently it simply goes off-screen.)
- Ensure that terminals and similar components are managed as windows.
- Make the terminal not a simplistic one; rewrite the documentation with bold and substantial changes so it becomes, as much as possible, a serious, full-scratch implementation.
- Autonomously find places where specification explanations in the documentation are insufficient, and explain them.
- Boldly and substantially rewrite the documentation so that multi-window state can be saved to and loaded from a JSON file.
</context>
