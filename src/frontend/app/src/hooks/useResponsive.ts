import { useState, useEffect } from 'react'

export function useResponsive() {
  const [isCompact, setIsCompact] = useState(window.innerWidth <= 1280)

  useEffect(() => {
    const handleResize = () => {
      setIsCompact(window.innerWidth <= 1280)
    }

    window.addEventListener('resize', handleResize)
    return () => window.removeEventListener('resize', handleResize)
  }, [])

  return { isCompact }
}
