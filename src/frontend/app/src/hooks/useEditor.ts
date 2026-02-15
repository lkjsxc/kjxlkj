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

interface UseEditorOptions {
  autosave?: boolean;
}

export function useEditor(options: UseEditorOptions = {}) {
  const autosave = options.autosave ?? true;
  const state = useEditorState();
  const editorDispatch = useEditorDispatch();
  const notesDispatch = useNotesDispatch();
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const saveInFlightRef = useRef(false);
  const saveQueuedRef = useRef(false);
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

  const saveOnce = useCallback(async (): Promise<"noop" | "saved" | "conflict" | "error"> => {
    const s = stateRef.current;
    if (!s.noteId || !s.dirty) return "noop";
    const saveNoteId = s.noteId;
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
          base_body: s.syncedBody,
          body: s.draftBody,
          expected_version: version,
        });
        version = n.version;
        notesDispatch({ type: "update_note", note: n });
      }
      if (stateRef.current.noteId === saveNoteId) {
        editorDispatch({
          type: "saved",
          title: s.draftTitle,
          body: s.draftBody,
          version,
        });
      }
      return "saved";
    } catch (err) {
      if (err instanceof ApiClientError && err.status === 409) {
        if (stateRef.current.noteId === saveNoteId) {
          editorDispatch({
            type: "conflict",
            serverVersion: (err.body.details?.["current_version"] as number) ?? 0,
          });
        }
        return "conflict";
      } else {
        if (stateRef.current.noteId === saveNoteId) {
          editorDispatch({ type: "error" });
        }
        return "error";
      }
    }
  }, [editorDispatch, notesDispatch]);

  const save = useCallback(async () => {
    if (saveInFlightRef.current) {
      saveQueuedRef.current = true;
      return;
    }
    saveInFlightRef.current = true;
    try {
      while (true) {
        saveQueuedRef.current = false;
        const outcome = await saveOnce();
        if (outcome !== "saved") break;
        if (!saveQueuedRef.current) break;
      }
    } finally {
      saveInFlightRef.current = false;
    }
  }, [saveOnce]);

  // Autosave debounce
  useEffect(() => {
    if (!autosave) return;
    if (!state.dirty) return;
    if (timerRef.current) clearTimeout(timerRef.current);
    timerRef.current = setTimeout(() => void save(), DEBOUNCE_MS);
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, [autosave, state.draftTitle, state.draftBody, state.dirty, save]);

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
