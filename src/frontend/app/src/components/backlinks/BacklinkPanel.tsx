import React, { useState, useEffect } from 'react'

interface BacklinkPanelProps {
  noteId: string
}

export function BacklinkPanel({ noteId }: BacklinkPanelProps) {
  const [backlinks, setBacklinks] = useState<Array<{
    source_note_id: string
    source_title: string
    link_text: string
    snippet: string
  }>>([])

  useEffect(() => {
    const fetchBacklinks = async () => {
      const response = await fetch(`/api/notes/${noteId}/backlinks`)
      if (response.ok) {
        const data = await response.json()
        setBacklinks(data.backlinks || [])
      }
    }

    fetchBacklinks()
  }, [noteId])

  return (
    <div className="backlink-panel">
      <h3>Backlinks ({backlinks.length})</h3>
      {backlinks.map((backlink) => (
        <div key={backlink.source_note_id} className="backlink-item">
          <div className="backlink-title">📄 {backlink.source_title}</div>
          <div className="backlink-snippet">{backlink.snippet}</div>
        </div>
      ))}
      {backlinks.length === 0 && (
        <div className="backlink-snippet">No backlinks</div>
      )}
    </div>
  )
}
