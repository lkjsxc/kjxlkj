import React from 'react'

export function Toolbar() {
  const handleFormat = (format: string) => {
    // Implement formatting logic
    console.log('Format:', format)
  }

  return (
    <div className="editor-toolbar">
      <button onClick={() => handleFormat('h1')} title="Heading 1">H1</button>
      <button onClick={() => handleFormat('h2')} title="Heading 2">H2</button>
      <button onClick={() => handleFormat('h3')} title="Heading 3">H3</button>
      <button onClick={() => handleFormat('bold')} title="Bold"><strong>B</strong></button>
      <button onClick={() => handleFormat('italic')} title="Italic"><em>I</em></button>
      <button onClick={() => handleFormat('code')} title="Code">{`</>`}</button>
      <button onClick={() => handleFormat('quote')} title="Quote">&gt;</button>
      <button onClick={() => handleFormat('list')} title="List">•</button>
      <button onClick={() => handleFormat('link')} title="Link">🔗</button>
    </div>
  )
}
