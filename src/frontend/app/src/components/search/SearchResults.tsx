import React, { useState, useEffect } from 'react'

interface SearchResultsProps {
  query: string
}

export function SearchResults({ query }: SearchResultsProps) {
  const [results, setResults] = useState<Array<{
    note_id: string
    title: string
    snippet: string
    updated_at: string
  }>>([])
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    const search = async () => {
      if (!query) return
      
      setLoading(true)
      const response = await fetch(`/api/search?q=${encodeURIComponent(query)}`)
      if (response.ok) {
        const data = await response.json()
        setResults(data.results || [])
      }
      setLoading(false)
    }

    const debounce = setTimeout(search, 300)
    return () => clearTimeout(debounce)
  }, [query])

  const highlightMatch = (text: string, query: string) => {
    if (!query) return text
    const parts = text.split(new RegExp(`(${query})`, 'gi'))
    return parts.map((part, i) =>
      part.toLowerCase() === query.toLowerCase() ? <mark key={i}>{part}</mark> : part
    )
  }

  return (
    <div className="search-results">
      <h2>Search Results</h2>
      {loading ? (
        <p>Searching...</p>
      ) : results.length === 0 ? (
        <p>No results found</p>
      ) : (
        results.map((result) => (
          <div key={result.note_id} className="search-result-item">
            <div className="search-result-title">
              {highlightMatch(result.title, query)}
            </div>
            <div className="search-result-snippet">
              {highlightMatch(result.snippet || '', query)}
            </div>
            <div className="search-result-meta">
              Updated: {new Date(result.updated_at).toLocaleDateString()}
            </div>
          </div>
        ))
      )}
    </div>
  )
}
