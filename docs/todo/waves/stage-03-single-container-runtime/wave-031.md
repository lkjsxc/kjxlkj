# Wave 031: Command Palette and Navigation UX

Back: [/docs/todo/waves/stage-03-single-container-runtime/README.md](/docs/todo/waves/stage-03-single-container-runtime/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] implement command palette action model and shortcuts -> [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [ ] wire create/open/move/tag/run-rule commands to APIs -> [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [ ] preserve deterministic success/failure feedback states -> [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [ ] ensure setup-locked state renders login-only UI without setup-like visuals -> [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)

## Verification Tasks

- [ ] run `E2E-03` and targeted keyboard-navigation checks -> [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [ ] run command failure-path scenarios -> [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [ ] run `E2E-11` login/setup presentation split checks -> [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)

## Evidence Placeholder

- [ ] `Check: command workflow integration + keyboard shortcut/shell marker + setup-lock deterministic checks` -> [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [ ] `Result: pass` -> [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [ ] `Proof: [/docs/log/audits/2026-02-13-stage-03-wave-031-command-palette.md](/docs/log/audits/2026-02-13-stage-03-wave-031-command-palette.md)`
