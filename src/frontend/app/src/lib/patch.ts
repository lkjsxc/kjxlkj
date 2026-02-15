/**
 * Patch operation helpers for note editing.
 * Generates deterministic patch ops from synced snapshot + draft.
 */
export interface PatchOp {
  op: string;
  path: string;
  value?: string;
}

/** Generate patch ops from synced body to draft body. */
export function diffBody(synced: string, draft: string): PatchOp[] {
  if (synced === draft) return [];
  return [{ op: "replace", path: "/body", value: draft }];
}

/** Generate patch ops from synced title to draft title. */
export function diffTitle(synced: string, draft: string): PatchOp[] {
  if (synced === draft) return [];
  return [{ op: "replace", path: "/title", value: draft }];
}

/** Combine title and body diffs into one patch set. */
export function buildPatch(
  syncedTitle: string,
  draftTitle: string,
  syncedBody: string,
  draftBody: string,
): PatchOp[] {
  return [
    ...diffTitle(syncedTitle, draftTitle),
    ...diffBody(syncedBody, draftBody),
  ];
}
