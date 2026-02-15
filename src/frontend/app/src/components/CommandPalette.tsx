/**
 * CommandPalette: keyboard-first action palette.
 * Spec: UX-NAV-02 — keyboard-first create/open/move/tag.
 * Placeholder — full implementation in later stages.
 */
import { useState, useEffect, useRef } from "react";

interface Props {
  open: boolean;
  onClose: () => void;
  onAction: (action: string, arg?: string) => void;
}

const ACTIONS = [
  { id: "create", label: "Create New Note" },
  { id: "search", label: "Search Notes" },
];

export function CommandPalette({ open, onClose, onAction }: Props) {
  const [query, setQuery] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (open) {
      setQuery("");
      inputRef.current?.focus();
    }
  }, [open]);

  if (!open) return null;

  const filtered = ACTIONS.filter((a) =>
    a.label.toLowerCase().includes(query.toLowerCase()),
  );

  return (
    <div style={styles.overlay} onClick={onClose}>
      <div style={styles.palette} onClick={(e) => e.stopPropagation()}>
        <input
          ref={inputRef}
          type="text"
          placeholder="Type a command…"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Escape") onClose();
          }}
          style={styles.input}
        />
        <div style={styles.list}>
          {filtered.map((a) => (
            <button
              key={a.id}
              onClick={() => {
                onAction(a.id);
                onClose();
              }}
              style={styles.item}
            >
              {a.label}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}

const styles: Record<string, React.CSSProperties> = {
  overlay: {
    position: "fixed",
    inset: 0,
    background: "rgba(0,0,0,0.3)",
    display: "flex",
    alignItems: "flex-start",
    justifyContent: "center",
    paddingTop: "15vh",
    zIndex: 200,
  },
  palette: {
    background: "#fff",
    borderRadius: "8px",
    boxShadow: "0 4px 20px rgba(0,0,0,0.2)",
    width: "min(500px, 90vw)",
    overflow: "hidden",
  },
  input: {
    width: "100%",
    padding: "0.75rem 1rem",
    border: "none",
    borderBottom: "1px solid #eee",
    fontSize: "1rem",
    outline: "none",
  },
  list: { maxHeight: "40vh", overflowY: "auto" },
  item: {
    display: "block",
    width: "100%",
    padding: "0.6rem 1rem",
    border: "none",
    background: "transparent",
    textAlign: "left" as const,
    cursor: "pointer",
    fontSize: "0.9rem",
  },
};
