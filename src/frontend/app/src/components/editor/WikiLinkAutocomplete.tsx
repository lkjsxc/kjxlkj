import React, { useState, useEffect } from 'react'

interface WikiLinkAutocompleteProps {
  query: string
  position: { top: number; left: number }
  onSelect: (noteId: string, noteTitle: string) => void
  onClose: () => void
}

export function WikiLinkAutocomplete({
  query,
  position,
  onSelect,
  onClose,
}: WikiLinkAutocompleteProps) {
  const [selectedIndex, setSelectedIndex] = useState(0)
  const [suggestions, setSuggestions] = useState<Array<{ note_id: string; title: string }>>([])

  useEffect(() => {
    // Fetch matching notes
    const fetchSuggestions = async () => {
      const response = await fetch(`/api/notes?q=${encodeURIComponent(query)}`)
      if (response.ok) {
        const data = await response.json()
        setSuggestions(data.notes || [])
      }
    }

    fetchSuggestions()
  }, [query])

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      setSelectedIndex((prev) => (prev + 1) % suggestions.length)
    } else if (e.key === 'ArrowUp') {
      setSelectedIndex((prev) => (prev - 1 + suggestions.length) % suggestions.length)
    } else if (e.key === 'Enter') {
      const selected = suggestions[selectedIndex]
      if (selected) {
        onSelect(selected.note_id, selected.title)
      }
    } else if (e.key === 'Escape') {
      onClose()
    }
  }

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown as any)
    return () => window.removeEventListener('keydown', handleKeyDown as any)
  }, [selectedIndex, suggestions])

  return (
    <div
      className="wiki-autocomplete"
      style={{ top: position.top, left: position.left }}
    >
      {suggestions.map((suggestion, index) => (
        <div
          key={suggestion.note_id}
          className={`wiki-autocomplete-item ${index === selectedIndex ? 'active' : ''}`}
          onClick={() => onSelect(suggestion.note_id, suggestion.title)}
        >
          📄 {suggestion.title}
        </div>
      ))}
      {suggestions.length === 0 && (
        <div className="wiki-autocomplete-item">
          ➕ Create "{query}"
        </div>
      )}
    </div>
  )
}
