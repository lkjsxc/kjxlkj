# Current TODO

Back: [/docs/todo/README.md](/docs/todo/README.md)

All checkboxes below are intentionally unchecked for next full reconstruction.

## Completion Preconditions

- [ ] Feature is reachable from runtime entrypoint via real user input
- [ ] Deterministic tests exist at required layers
- [ ] `/docs/reference/CONFORMANCE.md` updated
- [ ] `/docs/reference/LIMITATIONS.md` updated for any remaining user-visible gaps
- [ ] Relevant links in `/docs/todo/doc-coverage/` confirmed

## Area Checklists

- [ ] [architecture.md](architecture.md)
- [ ] [editor-core.md](editor-core.md)
- [ ] [modes.md](modes.md)
- [ ] [editing.md](editing.md)
- [ ] [commands.md](commands.md)
- [ ] [features-core.md](features-core.md)
- [ ] [features-services.md](features-services.md)
- [ ] [features-editing.md](features-editing.md)
- [ ] [scripting.md](scripting.md)
- [ ] [ui-rendering.md](ui-rendering.md)
- [ ] [ux.md](ux.md)
- [ ] [technical.md](technical.md)
- [ ] [verification.md](verification.md)

## Critical Gap Closures

- [ ] Fix `A` / `Shift+a` append-at-EOL behavior per [/docs/spec/ux/keybindings/mode-entry.md](/docs/spec/ux/keybindings/mode-entry.md)
- [ ] Wire `:terminal` into real PTY-backed terminal windows per [/docs/spec/features/terminal/terminal.md](/docs/spec/features/terminal/terminal.md)
- [ ] Implement spatially correct split/window navigation graph per [/docs/spec/features/window/splits-windows.md](/docs/spec/features/window/splits-windows.md)
- [ ] Wire explorer toggle/open/split workflows per [/docs/spec/features/navigation/file_explorer.md](/docs/spec/features/navigation/file_explorer.md)
- [ ] Implement real filesystem-backed `:w` and `:e` behavior per [/docs/spec/commands/file/write-commands.md](/docs/spec/commands/file/write-commands.md)
- [ ] Wire `:SessionSave`/`:SessionLoad` command path per [/docs/spec/features/session/sessions.md](/docs/spec/features/session/sessions.md)
- [ ] Complete Japanese IME intercept/commit/cancel per [/docs/spec/modes/insert/input/insert-japanese-ime.md](/docs/spec/modes/insert/input/insert-japanese-ime.md)
- [ ] Guarantee wrapped rendering for long lines per [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)

## Direct-Link Coverage

- [ ] [/docs/todo/doc-coverage/README.md](/docs/todo/doc-coverage/README.md)

## Related

- Reconstruction contract: [/docs/todo/RECONSTRUCTION_PROMPT.md](/docs/todo/RECONSTRUCTION_PROMPT.md)
- Current conformance: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Current limitations: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
