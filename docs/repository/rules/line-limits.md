# Line Limits Contract

## Hard Limits

- Docs files stay at `<= 300` lines.
- Authored source files stay at `<= 200` lines.
- `quality check-lines` is the mandatory enforcement gate.
- New exclusions require an update to [../layout/src-layout.md](../layout/src-layout.md) before tooling changes.

## Headroom Targets

- Actively edited authored source files should target `<= 180` lines when a cohesive split is available.
- Authored source files above `190` lines are refactor candidates even when the hard gate passes.
- Do not compress formatting, merge unrelated logic, or remove clarity only to satisfy line counts.
- Prefer small cohesive helper modules over dense multi-purpose files.

## Documentation Shape

- Split docs by contract ownership rather than by arbitrary line count alone.
- Keep each docs directory as a `README.md` table of contents plus multiple child files.
- Update parent TOCs in the same batch as a docs split.
