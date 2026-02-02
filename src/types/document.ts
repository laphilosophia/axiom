export type DocumentStatus = 'draft' | 'active' | 'superseded' | 'archived'

export type DocumentRelationship = 'supersedes' | 'references'

export interface Document {
  id: string
  title: string
  content: string
  status: DocumentStatus
  path: string
  tags: string[]
  createdAt: string
  updatedAt: string
  contentHash?: string
  embedding?: number[]
  relationships?: {
    supersedes: string[]
    references: string[]
    supersededBy?: string
    referencedBy: string[]
  }
}

export interface SimilarityResult {
  document: Document
  similarity: number
  reason: string
}

export interface SearchFilters {
  status?: DocumentStatus[]
  tags?: string[]
  query?: string
}

export interface SearchResult {
  document: Document
  highlights: string[]
  score: number
}
