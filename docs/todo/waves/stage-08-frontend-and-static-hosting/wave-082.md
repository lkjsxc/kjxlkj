# Wave 082: Responsive Librarian UX and Keyboard Flows

Back: [/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] make librarian panels usable at 320px widths with no mode fork
- [ ] ensure keyboard-first launch and review flows via command palette
- [ ] preserve menu-toggle collapse/restore behavior during librarian sessions

## Verification Tasks

- [ ] run 320px interaction tests for librarian views
- [ ] run command-palette invocation and focus-trap checks

## Evidence Placeholder

- [ ] `Check:` `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test ui_shell --test command_workflow -- --nocapture`
- [ ] `Result:` pass
- [ ] `Proof:` responsive librarian action layout + command palette `run-rule`/`review-run` keyboard flow hooks + menu-collapse behavior validated in shell/command coverage
