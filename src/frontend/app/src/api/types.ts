export interface Note {
  note_id: string
  title: string
  markdown: string
  version: number
  workspace_id: string
  project_id?: string
  note_kind: string
  access_scope: string
  created_at: string
  updated_at: string
  deleted_at?: string
}

export interface NoteSummary {
  note_id: string
  title: string
  updated_at: string
}

export interface SearchQuery {
  q: string
  workspace_id: string
  project_id?: string
  mode?: 'hybrid' | 'lexical' | 'semantic'
  limit?: number
  offset?: number
  note_kind?: string
  sort?: 'relevance' | 'updated_at'
}

export interface SearchResult {
  note_id: string
  title: string
  snippet?: string
  score_lexical: number
  score_semantic: number
  score_rrf: number
  score_final: number
  backlink_count: number
  updated_at: string
  note_kind: string
  workspace_id: string
}

export interface SearchResponse {
  results: SearchResult[]
  total: number
  mode: string
  degraded: boolean
  degraded_reason?: string
  query_normalized: string
  query_expanded: string[]
  timing_ms: {
    lexical: number
    semantic: number
    fusion: number
    rerank: number
    total: number
  }
}

export interface Backlink {
  source_note_id: string
  source_title: string
  link_text: string
  snippet?: string
  updated_at: string
}

export interface BacklinkResponse {
  note_id: string
  backlinks: Backlink[]
  total: number
}

export interface DomainEvent {
  event_id: string
  note_id: string
  event_type: string
  event_seq: number
  version: number
  actor: {
    type: 'user' | 'agent'
    user_id?: string
    agent_run_id?: string
  }
  timestamp: string
  payload: Record<string, unknown>
}

export interface Workspace {
  workspace_id: string
  name: string
  description?: string
  owner_id: string
  created_at: string
  updated_at: string
}

export interface Project {
  project_id: string
  workspace_id: string
  name: string
  description?: string
  created_at: string
  updated_at: string
}

export interface User {
  user_id: string
  email: string
}

export interface Session {
  user_id: string
  email: string
}
