import { useEffect, useRef, useCallback } from 'react'
import { useSelector, useDispatch } from 'react-redux'
import { RootState } from '../state/store'
import { saveNote } from '../state/noteSlice'

const AUTOSAVE_DEBOUNCE_MS = 600

export function useAutosave() {
  const dispatch = useDispatch()
  const currentNote = useSelector((state: RootState) => state.note.currentNote)
  const hasUnsavedChanges = useSelector((state: RootState) => state.note.hasUnsavedChanges)
  const timeoutRef = useRef<NodeJS.Timeout | null>(null)

  const debouncedSave = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current)
    }

    timeoutRef.current = setTimeout(() => {
      if (currentNote && hasUnsavedChanges) {
        dispatch(saveNote(currentNote) as any)
      }
    }, AUTOSAVE_DEBOUNCE_MS)
  }, [dispatch, currentNote, hasUnsavedChanges])

  useEffect(() => {
    if (currentNote && hasUnsavedChanges) {
      debouncedSave()
    }

    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current)
      }
    }
  }, [currentNote, hasUnsavedChanges, debouncedSave])
}
