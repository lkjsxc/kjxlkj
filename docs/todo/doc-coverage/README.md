# Doc Coverage: Full Direct-Link Checklist

Back: [/docs/todo/README.md](/docs/todo/README.md)

This checklist set directly links every documentation file currently in the repository.

## Scope

- Includes root `README.md` and all `docs/**/*.md` files.
- Includes TODO documents themselves.
- Uses unchecked boxes by default for standby reconstruction mode.

## Parts

- [ ] [doc-coverage-1.md](doc-coverage-1.md)
- [ ] [doc-coverage-2.md](doc-coverage-2.md)
- [ ] [doc-coverage-3.md](doc-coverage-3.md)
- [ ] [doc-coverage-4.md](doc-coverage-4.md)
- [ ] [doc-coverage-5.md](doc-coverage-5.md)

## Regeneration Rule

When documentation files are added, removed, or renamed:

- regenerate these part files from filesystem inventory
- keep direct links absolute (`/docs/...` or `/README.md`)
- keep all entries unchecked until explicitly verified

## Verification Rule

Before finishing a reconstruction wave:

- [ ] confirm no documentation file is missing from these parts
- [ ] confirm no stale links remain after log cleanup
