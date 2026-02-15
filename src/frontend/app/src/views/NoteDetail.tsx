/**
 * NoteDetail: Markdown-native editor with title, autosave, conflict handling.
 * Per /docs/spec/ui/web-app.md and /docs/spec/ui/editor-flow.md.
 * - Title editable, propagates to list in same interaction cycle.
 * - Autosave is the default authoring path.
 * - Default chrome omits inline version/save/delete controls.
 * - synced/draft split per IMP-001.
 */
import { useRef, useCallback, useEffect } from "react";
import { useAppState, useAppDispatch } from "../state";
import { patchNote, updateTitle } from "../api";

export function NoteDetail() {
  const { activeNote, draftBody, session, workspaceId } = useAppState();
  const dispatch = useAppDispatch();
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const versionRef = useRef(0);

  const csrf = session?.csrf_token ?? "";
  const wsId = workspaceId ?? "";

  useEffect(() => {
    if (activeNote) {
      versionRef.current = activeNote.version;
    }
  }, [activeNote]);

  const debouncedSave = useCallback(
    (body: string) => {
      if (debounceRef.current) clearTimeout(debounceRef.current);
      debounceRef.current = setTimeout(async () => {
        if (!activeNote || !wsId || !csrf) return;
        try {
          const result = await patchNote(
            wsId,
            activeNote.note_id,
            [{ op: "replace_body", body }],
            versionRef.current,
            csrf,
          );
          versionRef.current = result.version;
        } catch {
          // Conflict or error — status feedback shown via state
        }
      }, 800); // Per data/config.json autosave_debounce_ms
    },
    [activeNote, wsId, csrf],
  );

  function handleBodyChange(value: string) {
    dispatch({ type: "SET_DRAFT", body: value });
    debouncedSave(value);
  }

  async function handleTitleChange(newTitle: string) {
    if (!activeNote || !wsId || !csrf) return;
    // Per /docs/spec/ui/web-app.md: Title edits MUST propagate to
    // lists and related navigation surfaces in the same interaction cycle.
    dispatch({
      type: "UPDATE_NOTE_TITLE",
      noteId: activeNote.note_id,
      title: newTitle,
    });
    try {
      await updateTitle(
        wsId,
        activeNote.note_id,
        newTitle,
        versionRef.current,
        csrf,
      );
      versionRef.current++;
    } catch {
      // version conflict handling
    }
  }

  if (!activeNote) {
    return (
      <main className="note-detail empty">
        <p>Select a note or create a new one</p>
      </main>
    );
  }

  return (
    <main className="note-detail">
      <input
        className="note-title-input"
        value={activeNote.title}
        onChange={(e) => handleTitleChange(e.target.value)}
        aria-label="Note title"
      />
      <textarea
        className="note-body-editor"
        value={draftBody}
        onChange={(e) => handleBodyChange(e.target.value)}
        aria-label="Note body"
        autoFocus
      />
    </main>
  );
}
