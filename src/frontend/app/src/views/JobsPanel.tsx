/**
 * JobsPanel: export/backup/automation progress.
 * Placeholder — full implementation in later stages.
 */
export function JobsPanel() {
  return (
    <div style={styles.container}>
      <h2 style={styles.title}>Jobs</h2>
      <p style={styles.empty}>No active jobs</p>
    </div>
  );
}

const styles: Record<string, React.CSSProperties> = {
  container: { padding: "1rem" },
  title: { fontSize: "1.125rem", marginBottom: "0.5rem" },
  empty: { color: "#999" },
};
