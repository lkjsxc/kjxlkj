/**
 * JobsPanel: export/backup/automation progress.
 * Spec: web-app.md §Required Shell Views — jobs panel.
 * Shows real-time automation run status and librarian review.
 */
import { useState, useEffect, useCallback } from "react";
import { listRuns, type AutomationRun } from "../api/automation";

interface Props {
  workspaceId: string;
  onReview?: (runId: string) => void;
}

const STATUS_LABELS: Record<string, string> = {
  queued: "Queued",
  running: "Running…",
  succeeded: "Completed",
  failed: "Failed",
};

const STATUS_COLORS: Record<string, string> = {
  queued: "#ff9800",
  running: "#1976d2",
  succeeded: "#4caf50",
  failed: "#d32f2f",
};

export function JobsPanel({ workspaceId, onReview }: Props) {
  const [runs, setRuns] = useState<AutomationRun[]>([]);
  const [loading, setLoading] = useState(false);

  const load = useCallback(async () => {
    setLoading(true);
    try {
      const data = await listRuns(workspaceId);
      setRuns(data);
    } finally {
      setLoading(false);
    }
  }, [workspaceId]);

  useEffect(() => {
    void load();
  }, [load]);

  return (
    <div style={styles.container}>
      <h2 style={styles.title}>Jobs</h2>
      {loading && <p style={styles.loading}>Loading…</p>}
      {!loading && runs.length === 0 && (
        <p style={styles.empty}>No active jobs</p>
      )}
      {runs.map((run) => (
        <div key={run.id} style={styles.item}>
          <div style={styles.row}>
            <span
              style={{
                ...styles.dot,
                background: STATUS_COLORS[run.status] ?? "#999",
              }}
            />
            <span style={styles.status}>
              {STATUS_LABELS[run.status] ?? run.status}
            </span>
            <span style={styles.runId}>{run.id.slice(0, 8)}</span>
          </div>
          {run.status === "succeeded" && onReview && (
            <button
              onClick={() => onReview(run.id)}
              style={styles.reviewBtn}
            >
              Review
            </button>
          )}
        </div>
      ))}
    </div>
  );
}

const styles: Record<string, React.CSSProperties> = {
  container: { padding: "1rem" },
  title: { fontSize: "1.125rem", marginBottom: "0.75rem" },
  loading: { color: "#999" },
  empty: { color: "#999" },
  item: {
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    padding: "0.5rem 0",
    borderBottom: "1px solid #f0f0f0",
  },
  row: { display: "flex", alignItems: "center", gap: "0.4rem" },
  dot: { width: 8, height: 8, borderRadius: "50%", flexShrink: 0 },
  status: { fontSize: "0.875rem", fontWeight: 500 },
  runId: { fontSize: "0.75rem", color: "#999" },
  reviewBtn: {
    padding: "0.2rem 0.5rem",
    border: "1px solid #1976d2",
    borderRadius: "3px",
    background: "transparent",
    color: "#1976d2",
    cursor: "pointer",
    fontSize: "0.75rem",
  },
};
