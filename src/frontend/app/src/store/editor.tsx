/**
 * Editor store: synced snapshot + local draft split.
 * Spec: UX-EDIT-01 — synced and draft MUST be separate.
 * Spec: UX-EDIT-02 — autosave with bounded debounce.
 */
import {
  createContext,
  useReducer,
  useContext,
  type ReactNode,
  type Dispatch,
} from "react";
import type { SaveStatus } from "../types";

interface EditorState {
  noteId: string | null;
  syncedTitle: string;
  syncedBody: string;
  syncedVersion: number;
  draftTitle: string;
  draftBody: string;
  dirty: boolean;
  status: SaveStatus;
}

type EditorAction =
  | { type: "load"; noteId: string; title: string; body: string; version: number }
  | { type: "draft_title"; title: string }
  | { type: "draft_body"; body: string }
  | { type: "saving" }
  | { type: "saved"; title: string; body: string; version: number }
  | { type: "conflict"; serverVersion: number }
  | { type: "offline" }
  | { type: "online" }
  | { type: "error" }
  | { type: "clear" };

function isDirty(s: EditorState, draftTitle: string, draftBody: string): boolean {
  return draftTitle !== s.syncedTitle || draftBody !== s.syncedBody;
}

function reducer(state: EditorState, action: EditorAction): EditorState {
  switch (action.type) {
    case "load":
      return {
        noteId: action.noteId,
        syncedTitle: action.title,
        syncedBody: action.body,
        syncedVersion: action.version,
        draftTitle: action.title,
        draftBody: action.body,
        dirty: false,
        status: "idle",
      };
    case "draft_title": {
      const dirty = isDirty(state, action.title, state.draftBody);
      return { ...state, draftTitle: action.title, dirty };
    }
    case "draft_body": {
      const dirty = isDirty(state, state.draftTitle, action.body);
      return { ...state, draftBody: action.body, dirty };
    }
    case "saving":
      return { ...state, status: "saving" };
    case "saved":
      return {
        ...state,
        syncedTitle: action.title,
        syncedBody: action.body,
        syncedVersion: action.version,
        dirty: isDirty(
          { ...state, syncedTitle: action.title, syncedBody: action.body },
          state.draftTitle,
          state.draftBody,
        ),
        status: "saved",
      };
    case "conflict":
      return { ...state, syncedVersion: action.serverVersion, status: "conflict" };
    case "offline":
      return { ...state, status: "offline" };
    case "online":
      return { ...state, status: state.dirty ? "idle" : "saved" };
    case "error":
      return { ...state, status: "error" };
    case "clear":
      return initial;
  }
}

const initial: EditorState = {
  noteId: null,
  syncedTitle: "",
  syncedBody: "",
  syncedVersion: 0,
  draftTitle: "",
  draftBody: "",
  dirty: false,
  status: "idle",
};

const EditorStateCtx = createContext<EditorState>(initial);
const EditorDispatchCtx = createContext<Dispatch<EditorAction>>(() => {});

export function EditorProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(reducer, initial);
  return (
    <EditorStateCtx.Provider value={state}>
      <EditorDispatchCtx.Provider value={dispatch}>
        {children}
      </EditorDispatchCtx.Provider>
    </EditorStateCtx.Provider>
  );
}

export function useEditorState(): EditorState {
  return useContext(EditorStateCtx);
}

export function useEditorDispatch(): Dispatch<EditorAction> {
  return useContext(EditorDispatchCtx);
}
