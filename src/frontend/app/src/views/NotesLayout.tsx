// Split-pane notes layout per UX-LAYOUT-05, UX-LAYOUT-06.
// >= 1024px: list left, editor right.
// < 1024px: editor primary, top-left menu reveals list.
// UX-LAYOUT-03: independent pane scrolling.
// UX-NAV-01: note-first baseline; no dashboard/workspace switcher.

import { useCallback, useEffect, useState } from 'react';
import { api } from '../api';
import type { SessionInfo, Workspace } from '../types';
import MenuToggle from '../components/MenuToggle';
import NotesList from './NotesList';
import NoteDetail from './NoteDetail';

interface Props {
  session: SessionInfo;
}

export default function NotesLayout({ session }: Props) {
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const [selectedNoteId, setSelectedNoteId] = useState<string | null>(null);
  const [workspace, setWorkspace] = useState<Workspace | null>(null);
  const [titleOverrides, setTitleOverrides] = useState(
    () => new Map<string, string>(),
  );

  // Load first workspace for the user
  useEffect(() => {
    api
      .get<Workspace[]>('/api/workspaces')
      .then((list) => {
        if (list.length > 0) setWorkspace(list[0] ?? null);
      })
      .catch(() => {
        /* no workspaces yet */
      });
  }, [session]);

  // On compact screens, selecting a note closes the sidebar
  const handleSelectNote = useCallback((noteId: string) => {
    setSelectedNoteId(noteId);
    setSidebarOpen(false);
  }, []);

  // UX-EDIT-04: title propagation in same cycle
  const handleTitleChange = useCallback(
    (noteId: string, newTitle: string) => {
      setTitleOverrides((prev) => {
        const next = new Map(prev);
        next.set(noteId, newTitle);
        return next;
      });
    },
    [],
  );

  // Create a new note
  async function handleCreateNote() {
    if (!workspace) return;
    try {
      const note = await api.post<{ note_id: string }>(
        `/api/workspaces/${workspace.id}/notes`,
        { title: 'Untitled', note_kind: 'markdown' },
      );
      setSelectedNoteId(note.note_id);
      setSidebarOpen(false);
    } catch {
      /* ignore */
    }
  }

  return (
    <div className="app-layout">
      {/* UX-LAYOUT-06: top-left menu button on compact screens */}
      <MenuToggle
        sidebarOpen={sidebarOpen}
        onToggle={() => setSidebarOpen((o) => !o)}
      />

      {/* Sidebar / navigation pane */}
      <aside className={`sidebar${sidebarOpen ? ' sidebar-open' : ''}`}>
        <div className="sidebar-header">
          <span className="sidebar-title">Notes</span>
          <button
            type="button"
            className="sidebar-new-btn"
            onClick={handleCreateNote}
            aria-label="Create new note"
          >
            +
          </button>
        </div>
        <NotesList
          workspaceId={workspace?.id ?? null}
          selectedNoteId={selectedNoteId}
          onSelectNote={handleSelectNote}
          titleOverrides={titleOverrides}
        />
      </aside>

      {/* Main content / editor pane */}
      <main className="main-content">
        {selectedNoteId ? (
          <NoteDetail
            noteId={selectedNoteId}
            onTitleChange={handleTitleChange}
          />
        ) : (
          <div className="no-note-selected">
            <p>Select or create a note to start editing.</p>
          </div>
        )}
      </main>
    </div>
  );
}
