/**
 * LibrarianReview: per-operation accept/reject review UI.
 * Spec: UX-LIB-01 — per-operation accept/reject decisions.
 * Spec: UX-LIB-02 — preserve unresolved local drafts.
 */
import { useState, useCallback } from "react";
import type { LibrarianOperation } from "../api/automation";
import { reviewRun } from "../api/automation";

interface Props {
  runId: string;
  operations: LibrarianOperation[];
  onComplete: () => void;
}

export function LibrarianReview({ runId, operations, onComplete }: Props) {
  const [decisions, setDecisions] = useState<Record<string, string>>(() =>
    Object.fromEntries(operations.map((op) => [op.id, "pending"])),
  );
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState("");

  const setDecision = useCallback((opId: string, decision: string) => {
    setDecisions((prev) => ({ ...prev, [opId]: decision }));
  }, []);

  const handleSubmit = useCallback(async () => {
    setError("");
    setSubmitting(true);
    try {
      await reviewRun(runId, decisions);
      onComplete();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Review failed");
    } finally {
      setSubmitting(false);
    }
  }, [runId, decisions, onComplete]);

  const allDecided = Object.values(decisions).every(
    (d) => d === "accept" || d === "reject",
  );

  return (
    <div style={styles.container}>
      <h3 style={styles.title}>Librarian Review</h3>
      {error && <p style={styles.error}>{error}</p>}
      <div style={styles.list}>
        {operations.map((op) => (
          <div key={op.id} style={styles.item}>
            <div style={styles.opInfo}>
              <span style={styles.kind}>{op.kind}</span>
              <span style={styles.confidence}>
                {Math.round(op.confidence * 100)}%
              </span>
            </div>
            <div style={styles.actions} role="group" aria-label={`Decision for ${op.kind}`}>
              <button
                onClick={() => setDecision(op.id, "accept")}
                style={decisions[op.id] === "accept" ? styles.acceptActive : styles.btn}
                aria-pressed={decisions[op.id] === "accept"}
              >
                Accept
              </button>
              <button
                onClick={() => setDecision(op.id, "reject")}
                style={decisions[op.id] === "reject" ? styles.rejectActive : styles.btn}
                aria-pressed={decisions[op.id] === "reject"}
              >
                Reject
              </button>
            </div>
          </div>
        ))}
      </div>
      <button
        onClick={() => void handleSubmit()}
        disabled={!allDecided || submitting}
        style={styles.submit}
      >
        {submitting ? "Submitting…" : "Submit Review"}
      </button>
    </div>
  );
}

const styles: Record<string, React.CSSProperties> = {
  container: { padding: "1rem" },
  title: { fontSize: "1.125rem", marginBottom: "0.75rem" },
  error: { color: "#d32f2f", fontSize: "0.875rem", marginBottom: "0.5rem" },
  list: { display: "flex", flexDirection: "column", gap: "0.5rem" },
  item: {
    display: "flex", justifyContent: "space-between",
    alignItems: "center", padding: "0.5rem",
    border: "1px solid #e0e0e0", borderRadius: "4px",
  },
  opInfo: { display: "flex", gap: "0.5rem", alignItems: "center" },
  kind: { fontWeight: 600, fontSize: "0.875rem" },
  confidence: { color: "#666", fontSize: "0.75rem" },
  actions: { display: "flex", gap: "0.25rem" },
  btn: {
    padding: "0.25rem 0.5rem", border: "1px solid #ccc",
    borderRadius: "3px", background: "transparent",
    cursor: "pointer", fontSize: "0.75rem",
  },
  acceptActive: {
    padding: "0.25rem 0.5rem", border: "1px solid #4caf50",
    borderRadius: "3px", background: "#e8f5e9",
    color: "#2e7d32", cursor: "pointer", fontSize: "0.75rem",
  },
  rejectActive: {
    padding: "0.25rem 0.5rem", border: "1px solid #d32f2f",
    borderRadius: "3px", background: "#ffebee",
    color: "#c62828", cursor: "pointer", fontSize: "0.75rem",
  },
  submit: {
    marginTop: "0.75rem", padding: "0.5rem 1rem",
    background: "#1976d2", color: "#fff", border: "none",
    borderRadius: "4px", cursor: "pointer", fontSize: "0.875rem",
  },
};
