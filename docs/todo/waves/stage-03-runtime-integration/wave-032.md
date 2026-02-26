# Wave 032: Responsive Split Layout and Compact Menu Behavior

Back: [/docs/todo/waves/stage-03-runtime-integration/README.md](/docs/todo/waves/stage-03-runtime-integration/README.md)

## Relevant Documents

- [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md)
- [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md)
- [/docs/spec/ui/editor-flow.md](/docs/spec/ui/editor-flow.md)
- [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [ ] restructure-step S03-W032-01: enforce desktop split view (`>=1024px`) with notes list on left and editor on right from [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) [doc-link](/docs/spec/ui/reconstruction-ux-requirements.md)
- [ ] restructure-step S03-W032-02: enforce compact view (`<1024px`) with editor as primary surface from [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) [doc-link](/docs/spec/ui/reconstruction-ux-requirements.md)
- [ ] restructure-step S03-W032-03: place top-right menu button in compact view and reveal note list on demand per [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) [doc-link](/docs/spec/ui/layout-and-interaction.md)
- [ ] restructure-step S03-W032-06: close compact menu after note selection per [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) [doc-link](/docs/spec/ui/layout-and-interaction.md)
- [ ] restructure-step S03-W032-04: enforce independent pane scrolling and `320px` no-horizontal-scroll behavior from [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) [doc-link](/docs/spec/ui/layout-and-interaction.md)
- [ ] restructure-step S03-W032-05: keep optional workspace modules from crowding editor flows per [/docs/spec/ui/workspace-suite.md](/docs/spec/ui/workspace-suite.md) [doc-link](/docs/spec/ui/workspace-suite.md)

## Verification Hooks

- [ ] restructure-step S03-W032-V01: run `E2E-12`, `E2E-19`, and `E2E-25` checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [ ] restructure-step S03-W032-V02: sync responsive UX status in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) [doc-link](/docs/reference/CONFORMANCE.md)

## Mandatory Build and Test Gate

- [ ] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [ ] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [ ] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
