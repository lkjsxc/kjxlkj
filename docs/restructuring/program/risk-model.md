# Risk Model

## Risk Classes

- `R1` Structural drift: broken TOCs or invalid directory shape.
- `R2` Semantic drift: inconsistent definitions across files.
- `R3` Verification drift: gate contracts diverge from executable reality.
- `R4` Traceability drift: acceptance IDs exist without evidence anchors.

## Mitigations

- `R1`: run structure audit on every major batch.
- `R2`: use singular canonical definitions and mapping docs.
- `R3`: document blocked states explicitly; never silently skip gates.
- `R4`: require acceptance IDs and completion checklist in every wave file.

## Escalation

- `R1`/`R2` unresolved at stage boundary: block transition.
- `R3` unresolved at final stage: block acceptance.
- `R4` unresolved in any wave: block wave closure.
