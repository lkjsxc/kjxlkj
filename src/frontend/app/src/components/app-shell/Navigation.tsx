import React, { useState, useEffect } from 'react'
import { useSelector, useDispatch } from 'react-redux'
import { useNavigate } from 'react-router-dom'
import { RootState } from '../../state/store'
import { selectNotes, fetchNotes, selectNote } from '../../state/noteSlice'
import { toggleNavigation } from '../../state/uiSlice'

interface NavigationProps {
  isOpen: boolean
}

export function Navigation({ isOpen }: NavigationProps) {
  const dispatch = useDispatch()
  const navigate = useNavigate()
  const notes = useSelector(selectNotes)
  const currentNoteId = useSelector((state: RootState) => state.note.currentNoteId)
  const [searchQuery, setSearchQuery] = useState('')

  useEffect(() => {
    dispatch(fetchNotes() as any)
  }, [dispatch])

  const filteredNotes = notes.filter(note =>
    note.title.toLowerCase().includes(searchQuery.toLowerCase())
  )

  const handleNoteSelect = (noteId: string) => {
    dispatch(selectNote(noteId) as any)
    navigate(`/notes/${noteId}`)
    // Close navigation on mobile
    if (window.innerWidth <= 1280) {
      dispatch(toggleNavigation())
    }
  }

  const handleCreateNote = async () => {
    // Create new note via API
    const response = await fetch('/api/notes', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ workspace_id: 'default' }),
    })
    
    if (response.ok) {
      const note = await response.json()
      handleNoteSelect(note.note_id)
    }
  }

  return (
    <nav className={`navigation ${isOpen ? 'open' : ''}`}>
      <div className="navigation-search">
        <input
          type="text"
          placeholder="Search notes..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
        />
      </div>
      <div className="note-list">
        {filteredNotes.map(note => (
          <div
            key={note.note_id}
            className={`note-list-item ${note.note_id === currentNoteId ? 'active' : ''}`}
            onClick={() => handleNoteSelect(note.note_id)}
          >
            <div className="note-list-item-title">{note.title}</div>
            <div className="note-list-item-date">
              {new Date(note.updated_at).toLocaleDateString()}
            </div>
          </div>
        ))}
      </div>
      <button className="create-note-btn" onClick={handleCreateNote}>
        + Create New Note
      </button>
    </nav>
  )
}
