# Operations

Use this subtree for runtime deployment, verification, release acceptance, and automation behavior.

## Read This Section When

- You need to boot or upgrade a live stack.
- You need the compose verification pipeline.
- You need the exact acceptance gates or CI command bundle.
- You need SeaweedFS and PostgreSQL persistence rules.

## Child Index

- [deployment/README.md](deployment/README.md): single-host compose deployment, runtime stack shape, first live login, and backup/update flow
- [verification/README.md](verification/README.md): compose pipeline, verification services, manual runbook, and browser checks
- [quality/README.md](quality/README.md): mandatory gates and authored-file rules
- [automation/README.md](automation/README.md): CLI and GitHub Actions automation contracts

## Start Here

- Deploy or boot locally: [deployment/single-host-compose.md](deployment/single-host-compose.md)
- Run full acceptance: [verification/compose-pipeline.md](verification/compose-pipeline.md)
- See hard acceptance gates: [quality/gates.md](quality/gates.md)
