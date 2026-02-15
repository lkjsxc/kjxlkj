/**
 * Notes hook: load, create, select, search actions.
 * Spec: Create New Note MUST create and move focus to that note.
 */
import { useCallback, useEffect } from "react";
import { useNotesState, useNotesDispatch } from "../store/notes";
import {
  listNotes,
  createNote as apiCreate,
  deleteNote as apiDelete,
  searchNotes,
} from "../api/notes";

const DEFAULT_WORKSPACE = "default";

export function useNotes(workspaceId?: string) {
  const state = useNotesState();
  const dispatch = useNotesDispatch();
  const wsId = workspaceId ?? DEFAULT_WORKSPACE;

  const load = useCallback(async () => {
    dispatch({ type: "set_loading", loading: true });
    const notes = await listNotes(wsId);
    dispatch({ type: "set_notes", notes });
  }, [wsId, dispatch]);

  useEffect(() => {
    void load();
  }, [load]);

  const create = useCallback(
    async (title: string) => {
      const note = await apiCreate({
        workspace_id: wsId,
        title,
      });
      dispatch({ type: "add_note", note });
      dispatch({ type: "select", id: note.id });
    },
    [wsId, dispatch],
  );

  const remove = useCallback(
    async (id: string) => {
      await apiDelete(id);
      dispatch({ type: "remove_note", id });
    },
    [dispatch],
  );

  const select = useCallback(
    (id: string | null) => dispatch({ type: "select", id }),
    [dispatch],
  );

  const search = useCallback(
    async (query: string) => {
      dispatch({ type: "set_search", query });
      if (!query.trim()) {
        await load();
        return;
      }
      dispatch({ type: "set_loading", loading: true });
      const notes = await searchNotes(wsId, query);
      dispatch({ type: "set_notes", notes });
    },
    [wsId, dispatch, load],
  );

  return { ...state, load, create, remove, select, search };
}
