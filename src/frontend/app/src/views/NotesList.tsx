/**
 * NotesList: Searchable note index within workspace scope.
 * Per /docs/spec/ui/web-app.md and /docs/spec/ui/layout-and-interaction.md.
 * - On small screens, selecting a note MUST close the menu.
 * - Create New Note MUST create a note and move focus to editor.
 */
import { useState, type FormEvent } from "react";
import { useAppState, useAppDispatch } from "../state";
import { createNote, getNote, listNotes } from "../api";

export function NotesList() {
  const { notes, activeNote, workspaceId, session } = useAppState();
  const dispatch = useAppDispatch();
  const [search, setSearch] = useState("");

  const csrf = session?.csrf_token ?? "";
  const wsId = workspaceId ?? "";

  const filtered = search
    ? notes.filter((n) =>
        n.title.toLowerCase().includes(search.toLowerCase()),
      )
    : notes;

  async function handleCreate(e: FormEvent) {
    e.preventDefault();
    if (!wsId || !csrf) return;
    const result = await createNote(wsId, "Untitled", "document", csrf);
    // Refresh list
    const updated = await listNotes(wsId, csrf);
    dispatch({ type: "SET_NOTES", notes: updated });
    // Select the new note
    const projection = await getNote(wsId, result.id, csrf);
    dispatch({ type: "SET_ACTIVE_NOTE", note: projection });
    // Close menu on small screens
    dispatch({ type: "CLOSE_MENU" });
  }

  async function handleSelect(noteId: string) {
    if (!wsId || !csrf) return;
    const projection = await getNote(wsId, noteId, csrf);
    dispatch({ type: "SET_ACTIVE_NOTE", note: projection });
    // Per /docs/spec/ui/layout-and-interaction.md:
    // Selecting a note from constrained-screen navigation MUST close the menu.
    dispatch({ type: "CLOSE_MENU" });
  }

  return (
    <nav className="notes-list" aria-label="Notes list">
      <div className="notes-list-header">
        <input
          type="search"
          placeholder="Search notes…"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          aria-label="Search notes"
        />
        <button
          onClick={handleCreate}
          className="create-note-btn"
          aria-label="Create New Note"
        >
          + New
        </button>
      </div>
      <ul>
        {filtered.map((note) => (
          <li
            key={note.id}
            className={
              activeNote?.note_id === note.id ? "note-item active" : "note-item"
            }
          >
            <button onClick={() => handleSelect(note.id)}>
              {note.title || "Untitled"}
            </button>
          </li>
        ))}
        {filtered.length === 0 && (
          <li className="note-item empty">No notes found</li>
        )}
      </ul>
    </nav>
  );
}
