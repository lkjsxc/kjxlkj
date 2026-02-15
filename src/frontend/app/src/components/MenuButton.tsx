/**
 * MenuButton: compact-view navigation toggle.
 * Spec: UX-LAYOUT-06 — top-right menu button in compact view.
 */
interface Props {
  open: boolean;
  onClick: () => void;
}

export function MenuButton({ open, onClick }: Props) {
  return (
    <button
      onClick={onClick}
      style={styles.button}
      aria-label={open ? "Close menu" : "Open menu"}
      aria-expanded={open}
    >
      {open ? "✕" : "☰"}
    </button>
  );
}

const styles = {
  button: {
    position: "fixed" as const,
    top: "0.5rem",
    right: "0.5rem",
    zIndex: 100,
    width: 36,
    height: 36,
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    background: "#fff",
    border: "1px solid #ccc",
    borderRadius: "6px",
    fontSize: "1.25rem",
    cursor: "pointer",
    boxShadow: "0 1px 4px rgba(0,0,0,0.12)",
  } satisfies React.CSSProperties,
};
