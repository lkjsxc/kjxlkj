/**
 * Idempotency key generation.
 * Per /docs/spec/ui/editor-flow.md: MUST work even when crypto.randomUUID
 * is unavailable (REG-USR-002).
 */
export function generateIdempotencyKey(): string {
  if (
    typeof crypto !== "undefined" &&
    typeof crypto.randomUUID === "function"
  ) {
    return crypto.randomUUID();
  }
  // Fallback: timestamp + random hex
  const ts = Date.now().toString(36);
  const rand = Math.random().toString(36).slice(2, 10);
  return `${ts}-${rand}`;
}
