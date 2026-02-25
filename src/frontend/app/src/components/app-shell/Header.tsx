import React from 'react'

interface HeaderProps {
  onMenuToggle: () => void
}

export function Header({ onMenuToggle }: HeaderProps) {
  return (
    <header className="header">
      <h1 className="header-title">kjxlkj</h1>
      <div className="header-actions">
        <div className="sync-status">
          <span className="sync-indicator"></span>
          <span>Saved</span>
        </div>
        <button className="menu-toggle" onClick={onMenuToggle} aria-label="Toggle menu">
          ☰
        </button>
      </div>
    </header>
  )
}
