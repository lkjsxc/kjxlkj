import { createSlice, PayloadAction } from '@reduxjs/toolkit'

export interface Note {
  note_id: string
  title: string
  markdown: string
  version: number
  created_at: string
  updated_at: string
}

interface NoteState {
  notes: Note[]
  currentNote: Note | null
  currentNoteId: string | null
  hasUnsavedChanges: boolean
  isAuthenticated: boolean
  isSetupComplete: boolean
  isLoading: boolean
  error: string | null
}

const initialState: NoteState = {
  notes: [],
  currentNote: null,
  currentNoteId: null,
  hasUnsavedChanges: false,
  isAuthenticated: false,
  isSetupComplete: false,
  isLoading: false,
  error: null,
}

const noteSlice = createSlice({
  name: 'note',
  initialState,
  reducers: {
    setNotes(state, action: PayloadAction<Note[]>) {
      state.notes = action.payload
    },
    addNote(state, action: PayloadAction<Note>) {
      state.notes.unshift(action.payload)
    },
    updateNote(state, action: PayloadAction<Partial<Note> & { note_id: string }>) {
      const index = state.notes.findIndex(n => n.note_id === action.payload.note_id)
      if (index !== -1) {
        state.notes[index] = { ...state.notes[index], ...action.payload }
        if (state.currentNoteId === action.payload.note_id) {
          state.currentNote = state.notes[index]
        }
      }
      state.hasUnsavedChanges = true
    },
    selectNote(state, action: PayloadAction<string>) {
      state.currentNoteId = action.payload
      const note = state.notes.find(n => n.note_id === action.payload)
      state.currentNote = note || null
    },
    deleteNote(state, action: PayloadAction<string>) {
      state.notes = state.notes.filter(n => n.note_id !== action.payload)
      if (state.currentNoteId === action.payload) {
        state.currentNoteId = null
        state.currentNote = null
      }
    },
    setAuthenticated(state, action: PayloadAction<boolean>) {
      state.isAuthenticated = action.payload
    },
    setSetupComplete(state, action: PayloadAction<boolean>) {
      state.isSetupComplete = action.payload
    },
    setLoading(state, action: PayloadAction<boolean>) {
      state.isLoading = action.payload
    },
    setError(state, action: PayloadAction<string | null>) {
      state.error = action.payload
    },
    clearUnsavedChanges(state) {
      state.hasUnsavedChanges = false
    },
  },
})

export const {
  setNotes,
  addNote,
  updateNote,
  selectNote,
  deleteNote,
  setAuthenticated,
  setSetupComplete,
  setLoading,
  setError,
  clearUnsavedChanges,
} = noteSlice.actions

export const selectNotes = (state: { note: NoteState }) => state.note.notes
export const selectCurrentNote = (state: { note: NoteState }) => state.note.currentNote

// Async thunks
export const fetchNotes = () => async (dispatch: any) => {
  dispatch(setLoading(true))
  try {
    const response = await fetch('/api/notes?workspace_id=default')
    if (response.ok) {
      const data = await response.json()
      dispatch(setNotes(data.notes || []))
      dispatch(setAuthenticated(true))
      dispatch(setSetupComplete(true))
    }
  } catch (err) {
    dispatch(setError('Failed to fetch notes'))
  } finally {
    dispatch(setLoading(false))
  }
}

export const saveNote = (note: Note) => async (dispatch: any) => {
  try {
    const response = await fetch(`/api/notes/${note.note_id}`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        base_version: note.version,
        markdown: note.markdown,
        title: note.title,
      }),
    })

    if (response.ok) {
      const updated = await response.json()
      dispatch(updateNote(updated))
      dispatch(clearUnsavedChanges())
    }
  } catch (err) {
    dispatch(setError('Failed to save note'))
  }
}

export default noteSlice.reducer
