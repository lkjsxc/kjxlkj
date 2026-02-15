/**
 * StatusBar: save/sync/conflict/offline indicator.
 * Spec: UX-FEEDBACK-01 — visible and unobtrusive.
 * Spec: UX-EDIT-06 — conflict provides explicit action paths.
 */
import type { SaveStatus } from "../types";

interface Props {
  status: SaveStatus;
  dirty: boolean;
  onReload: () => void;
}

const labels: Record<SaveStatus, string> = {
  idle: "",
  saving: "Saving…",
  saved: "Saved",
  conflict: "Conflict",
  offline: "Offline",
  error: "Error",
};

const colors: Record<SaveStatus, string> = {
  idle: "transparent",
  saving: "#ff9800",
  saved: "#4caf50",
  conflict: "#d32f2f",
  offline: "#9e9e9e",
  error: "#d32f2f",
};

export function StatusBar({ status, dirty, onReload }: Props) {
  const label = dirty && status === "idle" ? "Unsaved" : labels[status];
  const color = dirty && status === "idle" ? "#ff9800" : colors[status];

  if (!label) return null;

  return (
    <div style={{ ...styles.bar, color }} role="status" aria-live="polite">
      <span style={styles.dot(color)} />
      <span>{label}</span>
      {status === "conflict" && (
        <button onClick={onReload} style={styles.reloadBtn}>
          Reload latest
        </button>
      )}
    </div>
  );
}

const styles = {
  bar: {
    display: "flex",
    alignItems: "center",
    gap: "0.35rem",
    fontSize: "0.75rem",
    whiteSpace: "nowrap" as const,
  } satisfies React.CSSProperties,
  dot: (c: string): React.CSSProperties => ({
    width: 8,
    height: 8,
    borderRadius: "50%",
    background: c,
    flexShrink: 0,
  }),
  reloadBtn: {
    marginLeft: "0.5rem",
    padding: "0.15rem 0.4rem",
    fontSize: "0.7rem",
    border: "1px solid #d32f2f",
    borderRadius: "3px",
    background: "transparent",
    color: "#d32f2f",
    cursor: "pointer",
  } satisfies React.CSSProperties,
};
