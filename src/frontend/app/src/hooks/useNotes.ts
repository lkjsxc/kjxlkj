/**
 * Notes hook: load, create, select, search actions.
 * Spec: Create New Note MUST create and move focus to that note.
 */
import { useCallback, useEffect, useState } from "react";
import { useNotesState, useNotesDispatch } from "../store/notes";
import {
  listNotes,
  createNote as apiCreate,
  deleteNote as apiDelete,
  searchNotes,
} from "../api/notes";
import { createWorkspace, listWorkspaces } from "../api/workspaces";

export function useNotes(workspaceId?: string) {
  const state = useNotesState();
  const dispatch = useNotesDispatch();
  const [resolvedWorkspaceId, setResolvedWorkspaceId] = useState<
    string | null
  >(workspaceId ?? null);

  const resolveWorkspaceId = useCallback(async (): Promise<string | null> => {
    if (workspaceId) {
      setResolvedWorkspaceId(workspaceId);
      return workspaceId;
    }
    if (resolvedWorkspaceId) return resolvedWorkspaceId;

    const existing = await listWorkspaces();
    const first = existing[0];
    if (first) {
      const id = first.id;
      setResolvedWorkspaceId(id);
      return id;
    }

    const id = await createWorkspace({ slug: "default", name: "Default" });
    setResolvedWorkspaceId(id);
    return id;
  }, [workspaceId, resolvedWorkspaceId]);

  const load = useCallback(async () => {
    dispatch({ type: "set_loading", loading: true });
    try {
      const wsId = await resolveWorkspaceId();
      if (!wsId) {
        dispatch({ type: "set_notes", notes: [] });
        return;
      }
      const notes = await listNotes(wsId);
      dispatch({ type: "set_notes", notes });
    } catch {
      dispatch({ type: "set_notes", notes: [] });
    }
  }, [resolveWorkspaceId, dispatch]);

  useEffect(() => {
    void load();
  }, [load]);

  const create = useCallback(
    async (title: string) => {
      const wsId = await resolveWorkspaceId();
      if (!wsId) return;
      const note = await apiCreate({
        workspace_id: wsId,
        title,
      });
      dispatch({ type: "add_note", note });
      dispatch({ type: "select", id: note.id });
    },
    [resolveWorkspaceId, dispatch],
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
      try {
        const wsId = await resolveWorkspaceId();
        if (!wsId) {
          dispatch({ type: "set_notes", notes: [] });
          return;
        }
        const notes = await searchNotes(wsId, query);
        dispatch({ type: "set_notes", notes });
      } catch {
        dispatch({ type: "set_notes", notes: [] });
      }
    },
    [resolveWorkspaceId, dispatch, load],
  );

  return { ...state, load, create, remove, select, search };
}
