# Note Editor Flow

Back: [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Editor Model

The editor MUST be an Obsidian-like markdown workspace with:

- plain markdown source editing as first-class surface
- markdown preview support without leaving edit context
- wiki-link authoring (`[[note]]`) and backlink awareness
- keyboard-centric command patterns

## State Model

- Maintain separate synced snapshot and local draft buffer.
- Patch generation MUST use synced snapshot + deterministic diff.
- Local draft MUST survive reconnects and auth refresh transitions.

## Editing Rules

- Autosave is the default write path with bounded debounce.
- WS `apply_patch` is primary mutation path when connected.
- HTTP fallback MAY be used when WS is unavailable.
- Manual save MAY exist, but default chrome remains low-noise.
- Title edits MUST propagate to list/navigation in the same cycle.

## Obsidian-Like Behaviors

- Support markdown shortcuts (headings, lists, code fences, blockquotes).
- Preserve raw markdown fidelity; no destructive rich-text conversions.
- Support split or toggle preview mode using same note state.
- Preserve cursor/selection across autosave commits.

## Replay and Idempotency Rules

- Duplicate `idempotency_key` MUST replay existing commit identity.
- Reconnect MUST replay from acknowledged cursor before new patch submit.
- Stale cursor submits MUST yield deterministic error payloads.

## Conflict UX

| Condition | Required UX |
|---|---|
| `patch_rejected` or HTTP `409` | explicit conflict state with reload/reapply paths |
| reconnect after disconnect | replay missing events before submitting new patch |
| idempotent retransmit | show stable prior commit identity, not duplicate success |

## Related

- Web app shell: [web-app.md](web-app.md)
- Layout contract: [layout-and-interaction.md](layout-and-interaction.md)
- Notes domain: [/docs/spec/domain/notes.md](/docs/spec/domain/notes.md)
- Findings map: [findings-traceability.md](findings-traceability.md)
