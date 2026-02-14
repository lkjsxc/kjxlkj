# Wave 032: Responsive Split Layout and Compact Menu Behavior

Back: [/docs/todo/waves/stage-03-single-container-runtime/README.md](/docs/todo/waves/stage-03-single-container-runtime/README.md)

## Relevant Documents

- [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md)
- [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md)
- [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [ ] restructure-step S03-W032-01: enforce desktop split view (`>=1024px`) with notes list on left and editor on right from [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- [ ] restructure-step S03-W032-02: enforce compact view (`<1024px`) with editor as primary surface from [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- [ ] restructure-step S03-W032-03: place top-left menu button in compact view and reveal note list on demand per [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md)
- [ ] restructure-step S03-W032-04: enforce independent pane scrolling and `320px` no-horizontal-scroll behavior from [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md)
- [ ] restructure-step S03-W032-05: keep optional workspace modules from crowding editor flows per [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)

## Verification Hooks

- [ ] restructure-step S03-W032-V01: run `E2E-07`, `E2E-08`, `E2E-12`, and `E2E-19` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] restructure-step S03-W032-V02: sync responsive UX status in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
