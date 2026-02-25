import React from 'react'

interface OverlayProps {
  isVisible: boolean
  onClick: () => void
}

export function Overlay({ isVisible, onClick }: OverlayProps) {
  return (
    <div
      className={`overlay ${isVisible ? 'visible' : ''}`}
      onClick={onClick}
    />
  )
}
