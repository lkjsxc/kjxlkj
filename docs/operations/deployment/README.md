# Deployment

Use this subtree for the canonical local runtime model: one host running Docker Compose.

## Read This Section When

- You need the runtime stack shape and direct app environment contract.
- You need the exact boot, health-check, backup, or upgrade flow.
- You are handing off a fresh deployment to the first operator.

## Child Index

- [single-host-compose.md](single-host-compose.md): end-to-end deployment sequence from clone to verified running stack
- [compose-stack.md](compose-stack.md): compose files, services, boot behavior, and persistent state
- [runtime-configuration.md](runtime-configuration.md): direct app environment versus persisted operator settings
- [first-login-and-live-use.md](first-login-and-live-use.md): first admin, settings review, and first live resources
- [backup-and-updates.md](backup-and-updates.md): backup, restore, update, failure response, and shutdown
