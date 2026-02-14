export function librarianStatus(runId: string | null): string {
  return runId ? `active:${runId}` : "idle";
}
