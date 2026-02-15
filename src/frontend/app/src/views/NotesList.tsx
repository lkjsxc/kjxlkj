/**
 * NotesList: searchable note index within workspace.
 * Spec: Note-first, searchable note index.
 * Spec: Create New Note MUST create and move focus.
 */
import { useState, type FormEvent } from "react";
import type { Note } from "../types";

interface Props {
  notes: Note[];
  selectedId: string | null;
  loading: boolean;
  onSelect: (id: string) => void;
  onCreate: (title: string) => Promise<void>;
  onSearch: (query: string) => Promise<void>;
}

export function NotesList({
  notes,
  selectedId,
  loading,
  onSelect,
  onCreate,
  onSearch,
}: Props) {
  const [newTitle, setNewTitle] = useState("");
  const [query, setQuery] = useState("");

  const handleCreate = async (e: FormEvent) => {
    e.preventDefault();
    if (!newTitle.trim()) return;
    await onCreate(newTitle.trim());
    setNewTitle("");
  };

  const handleSearch = (value: string) => {
    setQuery(value);
    void onSearch(value);
  };

  return (
    <div style={styles.container}>
      <div style={styles.header}>
        <input
          type="text"
          placeholder="Search notes…"
          value={query}
          onChange={(e) => handleSearch(e.target.value)}
          style={styles.searchInput}
        />
        <form
          onSubmit={(e) => void handleCreate(e)}
          style={styles.createForm}
        >
          <input
            type="text"
            placeholder="New note title"
            value={newTitle}
            onChange={(e) => setNewTitle(e.target.value)}
            style={styles.createInput}
          />
          <button type="submit" style={styles.createBtn}>+</button>
        </form>
      </div>
      <div style={styles.list}>
        {loading && <p style={styles.loading}>Loading…</p>}
        {notes.map((note) => (
          <button
            key={note.id}
            onClick={() => onSelect(note.id)}
            style={{
              ...styles.item,
              background: note.id === selectedId ? "#e3f2fd" : "transparent",
            }}
          >
            <span style={styles.itemTitle}>
              {note.title || "Untitled"}
            </span>
            <span style={styles.itemDate}>
              {note.updated_at
                ? new Date(note.updated_at).toLocaleDateString()
                : ""}
            </span>
          </button>
        ))}
        {!loading && notes.length === 0 && (
          <p style={styles.empty}>No notes yet</p>
        )}
      </div>
    </div>
  );
}

const styles: Record<string, React.CSSProperties> = {
  container: {
    display: "flex",
    flexDirection: "column",
    height: "100%",
    borderRight: "1px solid #e0e0e0",
  },
  header: { padding: "0.5rem", borderBottom: "1px solid #e0e0e0" },
  searchInput: {
    width: "100%",
    padding: "0.4rem",
    border: "1px solid #ccc",
    borderRadius: "4px",
    fontSize: "0.875rem",
    marginBottom: "0.4rem",
  },
  createForm: { display: "flex", gap: "0.25rem" },
  createInput: {
    flex: 1,
    padding: "0.4rem",
    border: "1px solid #ccc",
    borderRadius: "4px",
    fontSize: "0.875rem",
  },
  createBtn: {
    padding: "0.4rem 0.75rem",
    background: "#1976d2",
    color: "#fff",
    border: "none",
    borderRadius: "4px",
    cursor: "pointer",
    fontSize: "1rem",
  },
  list: { flex: 1, overflowY: "auto" },
  item: {
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    width: "100%",
    padding: "0.6rem 0.75rem",
    border: "none",
    borderBottom: "1px solid #f0f0f0",
    cursor: "pointer",
    textAlign: "left" as const,
    fontSize: "0.875rem",
  },
  itemTitle: { fontWeight: 500 },
  itemDate: { color: "#999", fontSize: "0.75rem" },
  loading: { padding: "1rem", color: "#999", textAlign: "center" as const },
  empty: { padding: "1rem", color: "#999", textAlign: "center" as const },
};
