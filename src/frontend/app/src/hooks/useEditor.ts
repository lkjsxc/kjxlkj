/**
 * Editor hook: autosave with bounded debounce.
 * Spec: UX-EDIT-02 — autosave MUST be default with bounded debounce.
 * Spec: UX-EDIT-04 — title rename MUST propagate same cycle.
 */
import { useCallback, useEffect, useRef } from "react";
import { useEditorState, useEditorDispatch } from "../store/editor";
import { useNotesDispatch } from "../store/notes";
import { getNote, patchNote, patchTitle } from "../api/notes";
import { ApiClientError } from "../api/client";

const DEBOUNCE_MS = 800;

export function useEditor() {
  const state = useEditorState();
  const editorDispatch = useEditorDispatch();
  const notesDispatch = useNotesDispatch();
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const stateRef = useRef(state);
  stateRef.current = state;

  const loadNote = useCallback(
    async (id: string) => {
      const note = await getNote(id);
      editorDispatch({
        type: "load",
        noteId: note.id,
        title: note.title,
        body: note.body,
        version: note.version,
      });
    },
    [editorDispatch],
  );

  const save = useCallback(async () => {
    const s = stateRef.current;
    if (!s.noteId || !s.dirty) return;
    editorDispatch({ type: "saving" });
    try {
      let version = s.syncedVersion;
      if (s.draftTitle !== s.syncedTitle) {
        const n = await patchTitle(s.noteId, {
          title: s.draftTitle,
          expected_version: version,
        });
        version = n.version;
        // Propagate title change to notes list (same cycle)
        notesDispatch({ type: "update_note", note: n });
      }
      if (s.draftBody !== s.syncedBody) {
        const n = await patchNote(s.noteId, {
          body: s.draftBody,
          expected_version: version,
        });
        version = n.version;
        notesDispatch({ type: "update_note", note: n });
      }
      editorDispatch({
        type: "saved",
        title: s.draftTitle,
        body: s.draftBody,
        version,
      });
    } catch (err) {
      if (err instanceof ApiClientError && err.status === 409) {
        editorDispatch({
          type: "conflict",
          serverVersion: (err.body.details?.["current_version"] as number) ?? 0,
        });
      } else {
        editorDispatch({ type: "error" });
      }
    }
  }, [editorDispatch, notesDispatch]);

  // Autosave debounce
  useEffect(() => {
    if (!state.dirty) return;
    if (timerRef.current) clearTimeout(timerRef.current);
    timerRef.current = setTimeout(() => void save(), DEBOUNCE_MS);
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, [state.draftTitle, state.draftBody, state.dirty, save]);

  const setTitle = useCallback(
    (title: string) => editorDispatch({ type: "draft_title", title }),
    [editorDispatch],
  );

  const setBody = useCallback(
    (body: string) => editorDispatch({ type: "draft_body", body }),
    [editorDispatch],
  );

  const clear = useCallback(
    () => editorDispatch({ type: "clear" }),
    [editorDispatch],
  );

  const reloadLatest = useCallback(async () => {
    if (state.noteId) await loadNote(state.noteId);
  }, [state.noteId, loadNote]);

  return { ...state, loadNote, setTitle, setBody, save, clear, reloadLatest };
}
