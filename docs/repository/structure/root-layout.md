# Root Layout Contract

## Goal

- Define the final docs-only repository root without ambiguity.
- Keep the final authoritative state deterministic and verifiable.

## Final Authoritative Root (Required)

The version-controlled repository root MUST contain exactly these four entries:

1. `.gitignore`
2. `LICENSE`
3. `README.md`
4. `docs/`

## Final Deletion Contract (Required)

- Every tracked root entry not in the four-item keep-set MUST be deleted before the repository is considered final.
- Runtime/container implementation paths removed to reach final state include:
  - `.env.example`
  - `.github/`
  - `Cargo.lock`
  - `Cargo.toml`
  - `content/`
  - `data/`
  - `docker-compose.yml`
  - `migrations/`
  - `src/`
  - `static/`
  - `target/`
  - `templates/`

## Deletion Sequencing Rule

- Deletions MUST follow sequencing in [../governance/change-policy.md](../governance/change-policy.md).

## Deterministic Verification Checklist

1. Run: `git ls-tree --name-only HEAD | sort`.
2. Expected final output is exactly:
   - `.gitignore`
   - `LICENSE`
   - `README.md`
   - `docs`
3. If any extra entry appears, root finalization is incomplete.
