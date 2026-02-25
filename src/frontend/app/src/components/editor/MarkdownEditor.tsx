import React, { useState, useEffect, useRef } from 'react'
import CodeMirror from '@uiw/react-codemirror'
import { markdown } from '@codemirror/lang-markdown'
import { useSelector, useDispatch } from 'react-redux'
import { RootState } from '../../state/store'
import { updateNote } from '../../state/noteSlice'
import { WikiLinkAutocomplete } from './WikiLinkAutocomplete'
import { Toolbar } from './Toolbar'

interface MarkdownEditorProps {
  note: {
    note_id: string
    title: string
    markdown: string
    version: number
  }
}

export function MarkdownEditor({ note }: MarkdownEditorProps) {
  const dispatch = useDispatch()
  const [title, setTitle] = useState(note.title)
  const [content, setContent] = useState(note.markdown)
  const [showWikiAutocomplete, setShowWikiAutocomplete] = useState(false)
  const [wikiPosition, setWikiPosition] = useState<{ top: number; left: number } | null>(null)
  const [wikiQuery, setWikiQuery] = useState('')
  const editorRef = useRef<HTMLDivElement>(null)

  // Update local state when note changes
  useEffect(() => {
    setTitle(note.title)
    setContent(note.markdown)
  }, [note.note_id])

  const handleTitleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setTitle(e.target.value)
    dispatch(updateNote({ ...note, title: e.target.value }) as any)
  }

  const handleContentChange = (value: string) => {
    setContent(value)
    dispatch(updateNote({ ...note, markdown: value }) as any)

    // Check for wiki-link trigger
    const lines = value.split('\n')
    const cursorLine = lines.length - 1
    const lastLine = lines[cursorLine]
    const match = lastLine.match(/\[\[([^\]]*)$/)
    
    if (match) {
      setWikiQuery(match[1])
      setShowWikiAutocomplete(true)
      // Calculate position (simplified)
      setWikiPosition({ top: 100, left: 200 })
    } else {
      setShowWikiAutocomplete(false)
    }
  }

  const handleWikiSelect = (noteId: string, noteTitle: string) => {
    const newContent = content.replace(/\[\[[^\]]*$/, `[[${noteTitle}]]`)
    setContent(newContent)
    dispatch(updateNote({ ...note, markdown: newContent }) as any)
    setShowWikiAutocomplete(false)
  }

  return (
    <div className="editor-container" ref={editorRef}>
      <input
        type="text"
        className="editor-title-input"
        placeholder="Note title"
        value={title}
        onChange={handleTitleChange}
        aria-label="Note title"
      />
      <Toolbar />
      <div className="editor-body">
        <CodeMirror
          value={content}
          height="100%"
          extensions={[markdown()]}
          onChange={handleContentChange}
          basicSetup={{
            lineNumbers: false,
            foldGutter: false,
          }}
          theme="light"
        />
      </div>
      {showWikiAutocomplete && wikiPosition && (
        <WikiLinkAutocomplete
          query={wikiQuery}
          position={wikiPosition}
          onSelect={handleWikiSelect}
          onClose={() => setShowWikiAutocomplete(false)}
        />
      )}
    </div>
  )
}
