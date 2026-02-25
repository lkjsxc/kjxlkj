import React, { useState, useEffect } from 'react'
import { useSelector, useDispatch } from 'react-redux'
import { useNavigate, useLocation, useParams } from 'react-router-dom'
import { AppShell } from './components/app-shell/AppShell'
import { MarkdownEditor } from './components/editor/MarkdownEditor'
import { CommandPalette } from './components/command-palette/CommandPalette'
import { BacklinkPanel } from './components/backlinks/BacklinkPanel'
import { LoginForm } from './components/auth/LoginForm'
import { SetupForm } from './components/auth/SetupForm'
import { SearchResults } from './components/search/SearchResults'
import { useWebSocket } from './hooks/useWebSocket'
import { useAutosave } from './hooks/useAutosave'
import { RootState } from './state/store'
import { selectCurrentNote, saveNote } from './state/noteSlice'
import { toggleNavigation, toggleCommandPalette } from './state/uiSlice'

function App() {
  const dispatch = useDispatch()
  const navigate = useNavigate()
  const location = useLocation()
  const { id: noteId } = useParams<{ id: string }>()
  
  const currentNote = useSelector(selectCurrentNote)
  const isAuthenticated = useSelector((state: RootState) => state.note.isAuthenticated)
  const isSetupComplete = useSelector((state: RootState) => state.note.isSetupComplete)
  const showCommandPalette = useSelector((state: RootState) => state.ui.showCommandPalette)
  const isNavigationOpen = useSelector((state: RootState) => state.ui.isNavigationOpen)

  // WebSocket connection
  useWebSocket()

  // Autosave
  useAutosave()

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Cmd/Ctrl+P: Command palette
      if ((e.metaKey || e.ctrlKey) && e.key === 'p') {
        e.preventDefault()
        dispatch(toggleCommandPalette())
      }
      // Cmd/Ctrl+S: Manual save
      if ((e.metaKey || e.ctrlKey) && e.key === 's') {
        e.preventDefault()
        if (currentNote) {
          dispatch(saveNote(currentNote))
        }
      }
      // Escape: Close navigation/palette
      if (e.key === 'Escape') {
        if (showCommandPalette) {
          dispatch(toggleCommandPalette())
        } else if (isNavigationOpen) {
          dispatch(toggleNavigation())
        }
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [dispatch, currentNote, showCommandPalette, isNavigationOpen])

  // Auth/setup check
  if (!isSetupComplete) {
    return <SetupForm />
  }

  if (!isAuthenticated) {
    return <LoginForm />
  }

  // Search view
  if (location.pathname === '/search') {
    const query = new URLSearchParams(location.search).get('q') || ''
    return (
      <AppShell>
        <SearchResults query={query} />
      </AppShell>
    )
  }

  // Note editor or empty state
  return (
    <AppShell>
      {currentNote ? (
        <>
          <MarkdownEditor note={currentNote} />
          <BacklinkPanel noteId={currentNote.note_id} />
          {showCommandPalette && <CommandPalette />}
        </>
      ) : (
        <div className="empty-state">
          <h2>No note selected</h2>
          <p>Select a note from the list or create a new one</p>
        </div>
      )}
    </AppShell>
  )
}

export default App
