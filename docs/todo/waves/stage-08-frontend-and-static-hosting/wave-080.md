# Wave 080: Librarian Control Panel and Rule Authoring UX

Back: [/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [x] add librarian rule form for provider, model, and strict-mode settings
- [x] add run launch and run status panels in workspace shell
- [x] add deterministic validation feedback for invalid librarian configs

## Verification Tasks

- [x] run UX flow checks for create/edit/disable librarian rules
- [x] run provider-mode form boundary cases

## Evidence Placeholder

- [x] `Check:` `TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:32768/kjxlkj_test cargo test -p kjxlkj-server --test ui_shell --test automation_rules_api --test automation_run_flow -- --nocapture`
- [x] `Result:` pass
- [x] `Proof:` librarian panel markup + rule create/update/disable + invalid-provider validation passed with deterministic API/UI feedback
