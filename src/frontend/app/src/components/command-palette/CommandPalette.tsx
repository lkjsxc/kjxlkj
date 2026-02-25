import React, { useState, useEffect } from 'react'
import { useDispatch } from 'react-redux'
import { toggleCommandPalette } from '../../state/uiSlice'

export function CommandPalette() {
  const dispatch = useDispatch()
  const [query, setQuery] = useState('')
  const [selectedIndex, setSelectedIndex] = useState(0)

  const commands = [
    { id: 'create', label: 'Create note', shortcut: 'Ctrl+N' },
    { id: 'search', label: 'Search notes', shortcut: 'Ctrl+K' },
    { id: 'toggle-preview', label: 'Toggle preview', shortcut: 'Ctrl+Shift+P' },
    { id: 'export', label: 'Export markdown', shortcut: 'Ctrl+E' },
    { id: 'backlinks', label: 'Toggle backlinks', shortcut: 'Ctrl+Shift+B' },
    { id: 'history', label: 'Version history', shortcut: 'Ctrl+Shift+H' },
    { id: 'delete', label: 'Delete note', shortcut: 'Ctrl+Shift+D' },
  ]

  const filteredCommands = commands.filter(cmd =>
    cmd.label.toLowerCase().includes(query.toLowerCase())
  )

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      setSelectedIndex((prev) => (prev + 1) % filteredCommands.length)
    } else if (e.key === 'ArrowUp') {
      setSelectedIndex((prev) => (prev - 1 + filteredCommands.length) % filteredCommands.length)
    } else if (e.key === 'Enter') {
      const selected = filteredCommands[selectedIndex]
      if (selected) {
        executeCommand(selected.id)
      }
    } else if (e.key === 'Escape') {
      dispatch(toggleCommandPalette())
    }
  }

  const executeCommand = (commandId: string) => {
    console.log('Execute command:', commandId)
    dispatch(toggleCommandPalette())
  }

  return (
    <div className="command-palette">
      <input
        type="text"
        className="command-palette-input"
        placeholder="Type a command..."
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        onKeyDown={handleKeyDown}
        autoFocus
      />
      <div className="command-list">
        {filteredCommands.map((command, index) => (
          <div
            key={command.id}
            className={`command-item ${index === selectedIndex ? 'active' : ''}`}
            onClick={() => executeCommand(command.id)}
          >
            <span>{command.label}</span>
            <span className="command-shortcut">{command.shortcut}</span>
          </div>
        ))}
      </div>
    </div>
  )
}
