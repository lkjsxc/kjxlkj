/**
 * Notes list store.
 * Maintains the list of notes for the active workspace.
 */
import {
  createContext,
  useReducer,
  useContext,
  type ReactNode,
  type Dispatch,
} from "react";
import type { Note } from "../types";

interface NotesState {
  notes: Note[];
  loading: boolean;
  selectedId: string | null;
  searchQuery: string;
}

type NotesAction =
  | { type: "set_notes"; notes: Note[] }
  | { type: "set_loading"; loading: boolean }
  | { type: "select"; id: string | null }
  | { type: "set_search"; query: string }
  | { type: "add_note"; note: Note }
  | { type: "update_note"; note: Note }
  | { type: "remove_note"; id: string };

function reducer(state: NotesState, action: NotesAction): NotesState {
  switch (action.type) {
    case "set_notes":
      return { ...state, notes: action.notes, loading: false };
    case "set_loading":
      return { ...state, loading: action.loading };
    case "select":
      return { ...state, selectedId: action.id };
    case "set_search":
      return { ...state, searchQuery: action.query };
    case "add_note":
      return { ...state, notes: [action.note, ...state.notes] };
    case "update_note":
      return {
        ...state,
        notes: state.notes.map((n) =>
          n.id === action.note.id
            ? {
                ...n,
                ...action.note,
                created_at: action.note.created_at || n.created_at,
                updated_at: action.note.updated_at || n.updated_at,
              }
            : n,
        ),
      };
    case "remove_note":
      return {
        ...state,
        notes: state.notes.filter((n) => n.id !== action.id),
        selectedId:
          state.selectedId === action.id ? null : state.selectedId,
      };
  }
}

const initial: NotesState = {
  notes: [],
  loading: false,
  selectedId: null,
  searchQuery: "",
};

const NotesStateCtx = createContext<NotesState>(initial);
const NotesDispatchCtx = createContext<Dispatch<NotesAction>>(() => {});

export function NotesProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(reducer, initial);
  return (
    <NotesStateCtx.Provider value={state}>
      <NotesDispatchCtx.Provider value={dispatch}>
        {children}
      </NotesDispatchCtx.Provider>
    </NotesStateCtx.Provider>
  );
}

export function useNotesState(): NotesState {
  return useContext(NotesStateCtx);
}

export function useNotesDispatch(): Dispatch<NotesAction> {
  return useContext(NotesDispatchCtx);
}
