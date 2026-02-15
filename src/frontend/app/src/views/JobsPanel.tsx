/**
 * JobsPanel: Export/backup/automation progress including librarian runs.
 * Per /docs/spec/ui/web-app.md.
 */
export function JobsPanel() {
  return (
    <aside className="jobs-panel">
      <h2>Jobs</h2>
      <p className="jobs-empty">No active jobs</p>
    </aside>
  );
}
