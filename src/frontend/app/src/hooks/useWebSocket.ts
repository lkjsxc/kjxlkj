import { useEffect, useRef, useCallback } from 'react'
import { useDispatch } from 'react-redux'

export function useWebSocket() {
  const dispatch = useDispatch()
  const wsRef = useRef<WebSocket | null>(null)

  useEffect(() => {
    const connectWebSocket = () => {
      wsRef.current = new WebSocket('ws://localhost:8080/ws')

      wsRef.current.onopen = () => {
        console.log('WebSocket connected')
      }

      wsRef.current.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data)
          console.log('WebSocket message:', message)
          // Handle different message types
          switch (message.type) {
            case 'note_event':
              // Dispatch action to update note
              break
            case 'patch_committed':
              // Update sync status
              break
            case 'patch_rejected':
              // Handle conflict
              break
          }
        } catch (err) {
          console.error('Failed to parse WebSocket message:', err)
        }
      }

      wsRef.current.onclose = () => {
        console.log('WebSocket closed, reconnecting...')
        setTimeout(connectWebSocket, 1000)
      }

      wsRef.current.onerror = (error) => {
        console.error('WebSocket error:', error)
      }
    }

    connectWebSocket()

    return () => {
      if (wsRef.current) {
        wsRef.current.close()
      }
    }
  }, [dispatch])

  const sendMessage = useCallback((message: object) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message))
    }
  }, [])

  return { sendMessage }
}
