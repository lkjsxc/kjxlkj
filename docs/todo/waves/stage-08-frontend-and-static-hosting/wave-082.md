# Wave 082: Static Delivery, Responsive, and Accessibility Closure

Back: [/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md](/docs/todo/waves/stage-08-frontend-and-static-hosting/README.md)

## Relevant Documents

- [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
- [/docs/reference/CI.md](/docs/reference/CI.md)
- [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md)
- [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md)
- [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)

## Restructure Steps

- [ ] restructure-step S08-W082-01: satisfy static build/serve boundaries from [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md) [doc-link](/docs/spec/architecture/deployment.md)
- [ ] restructure-step S08-W082-02: satisfy wave build/test profile requirements from [/docs/reference/CI.md](/docs/reference/CI.md) [doc-link](/docs/reference/CI.md)
- [ ] restructure-step S08-W082-03: enforce responsive split/menu behavior from [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) [doc-link](/docs/spec/ui/layout-and-interaction.md)
- [ ] restructure-step S08-W082-04: enforce keyboard and accessibility requirements from [/docs/spec/ui/reconstruction-ux-requirements.md](/docs/spec/ui/reconstruction-ux-requirements.md) [doc-link](/docs/spec/ui/reconstruction-ux-requirements.md)
- [ ] restructure-step S08-W082-05: enforce strict frontend type gates from [/docs/spec/technical/type-safety.md](/docs/spec/technical/type-safety.md) [doc-link](/docs/spec/technical/type-safety.md)

## Verification Hooks

- [ ] restructure-step S08-W082-V01: run `E2E-19`, `E2E-21`, `E2E-22`, and `REG-UX-003` responsive/a11y checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)
- [ ] restructure-step S08-W082-V02: sync frontend delivery status in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) [doc-link](/docs/reference/CONFORMANCE.md)

## Mandatory Build and Test Gate

- [ ] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [ ] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [ ] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
