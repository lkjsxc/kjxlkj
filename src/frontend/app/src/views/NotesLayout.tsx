/**
 * NotesLayout: responsive split-pane shell.
 * Spec: UX-LAYOUT-01 — one responsive tree, no mode forks.
 * Spec: UX-LAYOUT-05 — >=1024px list left, editor right.
 * Spec: UX-LAYOUT-06 — <1024px editor primary, top-right menu.
 * Spec: UX-LAYOUT-07 — selecting note from menu MUST close menu.
 */
import { useState, useEffect, useCallback } from "react";
import { NotesList } from "./NotesList";
import { NoteDetail } from "./NoteDetail";
import { MenuButton } from "../components/MenuButton";
import { CommandPalette } from "../components/CommandPalette";
import { useNotes } from "../hooks/useNotes";
import { useEditor } from "../hooks/useEditor";

const BREAKPOINT = 1024;

export function NotesLayout() {
  const [menuOpen, setMenuOpen] = useState(false);
  const [paletteOpen, setPaletteOpen] = useState(false);
  const [compact, setCompact] = useState(window.innerWidth < BREAKPOINT);
  const {
    notes,
    selectedId,
    loading,
    select,
    create,
    search,
  } = useNotes();
  const { clear } = useEditor({ autosave: false });

  useEffect(() => {
    const onResize = () => setCompact(window.innerWidth < BREAKPOINT);
    window.addEventListener("resize", onResize);
    return () => window.removeEventListener("resize", onResize);
  }, []);

  // Keyboard shortcut: Ctrl+K for command palette
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "k") {
        e.preventDefault();
        setPaletteOpen((p) => !p);
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  const handleSelect = useCallback(
    (id: string) => {
      select(id);
      // Close menu on note selection (compact: UX-LAYOUT-07)
      if (compact) setMenuOpen(false);
    },
    [select, compact],
  );

  const handlePaletteAction = useCallback(
    (action: string) => {
      if (action === "create") void create("Untitled");
    },
    [create],
  );

  const handleDeselect = useCallback(() => {
    select(null);
    clear();
  }, [select, clear]);

  return (
    <div style={styles.root}>
      {/* Compact menu toggle */}
      {compact && (
        <MenuButton
          open={menuOpen}
          onClick={() => setMenuOpen((o) => !o)}
        />
      )}

      {/* Notes list pane */}
      {(!compact || menuOpen) && (
        <div style={compact ? styles.listCompact : styles.listDesktop}>
          <NotesList
            notes={notes}
            selectedId={selectedId}
            loading={loading}
            onSelect={handleSelect}
            onCreate={create}
            onSearch={search}
          />
        </div>
      )}

      {/* Editor pane */}
      <div style={styles.editor}>
        {selectedId ? (
          <NoteDetail noteId={selectedId} />
        ) : (
          <EmptyState onDeselect={handleDeselect} />
        )}
      </div>

      <CommandPalette
        open={paletteOpen}
        onClose={() => setPaletteOpen(false)}
        onAction={handlePaletteAction}
      />
    </div>
  );
}

function EmptyState({ onDeselect: _onDeselect }: { onDeselect: () => void }) {
  return (
    <div style={styles.empty}>
      <p>Select or create a note</p>
      <p style={styles.hint}>Ctrl+K for command palette</p>
    </div>
  );
}

const styles: Record<string, React.CSSProperties> = {
  root: { display: "flex", height: "100%", overflow: "hidden" },
  listDesktop: {
    width: 300,
    minWidth: 240,
    flexShrink: 0,
    overflowY: "auto",
  },
  listCompact: {
    position: "fixed",
    inset: 0,
    zIndex: 50,
    background: "#fff",
    overflowY: "auto",
    paddingTop: "3rem",
  },
  editor: { flex: 1, overflowY: "auto" },
  empty: {
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
    justifyContent: "center",
    height: "100%",
    color: "#999",
  },
  hint: { fontSize: "0.8rem", marginTop: "0.25rem" },
};
