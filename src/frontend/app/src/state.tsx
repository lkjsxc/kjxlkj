/**
 * Global application state.
 * Per /docs/spec/ui/editor-flow.md: separate synced snapshot from local draft.
 */
import {
  createContext,
  useContext,
  useReducer,
  type Dispatch,
  type ReactNode,
} from "react";
import type { SessionInfo, NoteListItem, NoteProjection } from "./api";

/* ---------- state shape ---------- */

export interface AppState {
  /** null = not loaded yet, undefined = unauthenticated */
  session: SessionInfo | null | undefined;
  /** Currently selected workspace id */
  workspaceId: string | null;
  /** Note list */
  notes: NoteListItem[];
  /** Active note projection (synced snapshot) */
  activeNote: NoteProjection | null;
  /** Local draft body (editor buffer) */
  draftBody: string;
  /** Whether data is loading */
  loading: boolean;
  /** Whether the compact-screen menu is open */
  menuOpen: boolean;
}

export const initialState: AppState = {
  session: null,
  workspaceId: null,
  notes: [],
  activeNote: null,
  draftBody: "",
  loading: true,
  menuOpen: false,
};

/* ---------- actions ---------- */

export type Action =
  | { type: "SET_SESSION"; session: SessionInfo | undefined }
  | { type: "SET_WORKSPACE"; workspaceId: string }
  | { type: "SET_NOTES"; notes: NoteListItem[] }
  | { type: "ADD_NOTE"; note: NoteListItem }
  | { type: "SET_ACTIVE_NOTE"; note: NoteProjection | null }
  | { type: "SET_DRAFT"; body: string }
  | { type: "SET_LOADING"; loading: boolean }
  | { type: "TOGGLE_MENU" }
  | { type: "CLOSE_MENU" }
  | { type: "UPDATE_NOTE_TITLE"; noteId: string; title: string };

/* ---------- reducer ---------- */

export function appReducer(state: AppState, action: Action): AppState {
  switch (action.type) {
    case "SET_SESSION":
      return { ...state, session: action.session, loading: false };
    case "SET_WORKSPACE":
      return { ...state, workspaceId: action.workspaceId };
    case "SET_NOTES":
      return { ...state, notes: action.notes };
    case "ADD_NOTE":
      return { ...state, notes: [action.note, ...state.notes] };
    case "SET_ACTIVE_NOTE":
      return {
        ...state,
        activeNote: action.note,
        draftBody: action.note?.body_text ?? "",
      };
    case "SET_DRAFT":
      return { ...state, draftBody: action.body };
    case "SET_LOADING":
      return { ...state, loading: action.loading };
    case "TOGGLE_MENU":
      return { ...state, menuOpen: !state.menuOpen };
    case "CLOSE_MENU":
      return { ...state, menuOpen: false };
    case "UPDATE_NOTE_TITLE": {
      const notes = state.notes.map((n) =>
        n.id === action.noteId ? { ...n, title: action.title } : n,
      );
      const activeNote =
        state.activeNote && state.activeNote.note_id === action.noteId
          ? { ...state.activeNote, title: action.title }
          : state.activeNote;
      return { ...state, notes, activeNote };
    }
    default:
      return state;
  }
}

/* ---------- context ---------- */

const StateCtx = createContext<AppState>(initialState);
const DispatchCtx = createContext<Dispatch<Action>>(() => {});

export function useAppState(): AppState {
  return useContext(StateCtx);
}

export function useAppDispatch(): Dispatch<Action> {
  return useContext(DispatchCtx);
}

export function AppProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(appReducer, initialState);
  return (
    <StateCtx.Provider value={state}>
      <DispatchCtx.Provider value={dispatch}>{children}</DispatchCtx.Provider>
    </StateCtx.Provider>
  );
}
