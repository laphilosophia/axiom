import { invoke } from '@tauri-apps/api/tauri'
import { AlertTriangle, GitBranch, Link, RefreshCw, Sparkles } from 'lucide-preact'
import { useEffect, useState } from 'preact/hooks'
import type { Document, SimilarityResult } from '../types/document'

interface OrchestrationPanelProps {
  document: Document | null
  onDocumentChange: () => void
}

export function OrchestrationPanel({ document, onDocumentChange }: OrchestrationPanelProps) {
  const [similarDocuments, setSimilarDocuments] = useState<SimilarityResult[]>([])
  const [isAnalyzing, setIsAnalyzing] = useState(false)
  const [relationships, setRelationships] = useState<{
    supersedes: Document[]
    references: Document[]
    supersededBy?: Document
    referencedBy: Document[]
  } | null>(null)

  useEffect(() => {
    if (document) {
      loadRelationships()
      analyzeSimilarity()
    } else {
      setSimilarDocuments([])
      setRelationships(null)
    }
  }, [document?.id])

  const loadRelationships = async () => {
    if (!document) return

    try {
      const rels = await invoke<{
        supersedes: Document[]
        references: Document[]
        supersededBy?: Document
        referencedBy: Document[]
      }>('get_document_relationships', { id: document.id })
      setRelationships(rels)
    } catch (error) {
      console.error('Failed to load relationships:', error)
    }
  }

  const analyzeSimilarity = async () => {
    if (!document || document.status !== 'active') return

    setIsAnalyzing(true)
    try {
      const results = await invoke<SimilarityResult[]>('find_similar_documents', {
        id: document.id,
        threshold: 0.75,
      })
      setSimilarDocuments(results)
    } catch (error) {
      console.error('Failed to analyze similarity:', error)
    } finally {
      setIsAnalyzing(false)
    }
  }

  const handleSupersede = async (targetId: string) => {
    if (!document) return

    try {
      await invoke('create_relationship', {
        fromId: document.id,
        toId: targetId,
        relationship: 'supersedes',
      })
      onDocumentChange()
      loadRelationships()
    } catch (error) {
      console.error('Failed to create relationship:', error)
    }
  }

  const handleReference = async (targetId: string) => {
    if (!document) return

    try {
      await invoke('create_relationship', {
        fromId: document.id,
        toId: targetId,
        relationship: 'references',
      })
      loadRelationships()
    } catch (error) {
      console.error('Failed to create relationship:', error)
    }
  }

  if (!document) {
    return <aside class="w-80 h-full glass-panel border-l border-border" />
  }

  return (
    <aside class="w-80 h-full glass-panel border-l border-border flex flex-col">
      {/* Header */}
      <div class="p-4 border-b border-border">
        <h2 class="text-sm font-semibold text-white flex items-center gap-2">
          <GitBranch className="w-4 h-4 text-accent-indigo" />
          Orchestration
        </h2>
      </div>

      <div class="flex-1 overflow-y-auto scrollbar-thin p-4 space-y-6">
        {/* Similar Documents */}
        <section>
          <div class="flex items-center justify-between mb-3">
            <h3 class="text-xs font-medium text-gray-400 uppercase tracking-wider flex items-center gap-2">
              <Sparkles className="w-3 h-3" />
              Similar Documents
            </h3>
            <button
              onClick={analyzeSimilarity}
              disabled={isAnalyzing}
              class="text-xs text-gray-500 hover:text-white disabled:opacity-50">
              <RefreshCw className={`w-3 h-3 ${isAnalyzing ? 'animate-spin' : ''}`} />
            </button>
          </div>

          {isAnalyzing ? (
            <div class="text-center py-4 text-gray-500 text-xs">
              <div class="w-4 h-4 mx-auto mb-2 border border-gray-500 border-t-transparent rounded-full animate-spin" />
              Analyzing...
            </div>
          ) : similarDocuments.length > 0 ? (
            <div class="space-y-2">
              {similarDocuments.map(({ document: simDoc, similarity, reason }) => (
                <div
                  key={simDoc.id}
                  class="p-3 rounded-lg bg-surface border border-border hover:border-accent-indigo/50 transition-all">
                  <div class="flex items-start justify-between">
                    <h4 class="text-sm font-medium text-white truncate pr-2">{simDoc.title}</h4>
                    <span class="text-xs font-mono text-accent-indigo">
                      {Math.round(similarity * 100)}%
                    </span>
                  </div>
                  <p class="text-xs text-gray-500 mt-1">{reason}</p>

                  <div class="flex gap-2 mt-2">
                    <button
                      onClick={() => handleSupersede(simDoc.id)}
                      class="text-xs px-2 py-1 bg-status-superseded/20 text-status-superseded rounded
                             hover:bg-status-superseded/30 transition-colors">
                      Supersede
                    </button>
                    <button
                      onClick={() => handleReference(simDoc.id)}
                      class="text-xs px-2 py-1 bg-accent-indigo/20 text-accent-indigo rounded
                             hover:bg-accent-indigo/30 transition-colors">
                      Reference
                    </button>
                  </div>
                </div>
              ))}
            </div>
          ) : document.status === 'active' ? (
            <div class="text-center py-4 text-gray-600 text-xs">
              <AlertTriangle className="w-4 h-4 mx-auto mb-1" />
              No similar documents found
            </div>
          ) : (
            <div class="text-center py-4 text-gray-600 text-xs">
              Analysis only available for active documents
            </div>
          )}
        </section>

        {/* Relationships */}
        <section>
          <h3 class="text-xs font-medium text-gray-400 uppercase tracking-wider mb-3 flex items-center gap-2">
            <Link className="w-3 h-3" />
            Relationships
          </h3>

          {relationships ? (
            <div class="space-y-4">
              {/* Superseded By */}
              {relationships.supersededBy && (
                <div>
                  <h4 class="text-xs text-gray-500 mb-1">Superseded By</h4>
                  <div class="p-2 rounded-lg bg-status-superseded/10 border border-status-superseded/30">
                    <span class="text-sm text-status-superseded">
                      {relationships.supersededBy.title}
                    </span>
                  </div>
                </div>
              )}

              {/* Supersedes */}
              {relationships.supersedes.length > 0 && (
                <div>
                  <h4 class="text-xs text-gray-500 mb-1">Supersedes</h4>
                  <div class="space-y-1">
                    {relationships.supersedes.map((d) => (
                      <div key={d.id} class="p-2 rounded-lg bg-surface text-sm text-gray-400">
                        {d.title}
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* References */}
              {relationships.references.length > 0 && (
                <div>
                  <h4 class="text-xs text-gray-500 mb-1">References</h4>
                  <div class="space-y-1">
                    {relationships.references.map((d) => (
                      <div key={d.id} class="p-2 rounded-lg bg-surface text-sm text-gray-400">
                        {d.title}
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Referenced By */}
              {relationships.referencedBy.length > 0 && (
                <div>
                  <h4 class="text-xs text-gray-500 mb-1">Referenced By</h4>
                  <div class="space-y-1">
                    {relationships.referencedBy.map((d) => (
                      <div key={d.id} class="p-2 rounded-lg bg-surface text-sm text-gray-400">
                        {d.title}
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* No Relationships */}
              {!relationships.supersededBy &&
                relationships.supersedes.length === 0 &&
                relationships.references.length === 0 &&
                relationships.referencedBy.length === 0 && (
                  <p class="text-xs text-gray-600 text-center py-2">No relationships yet</p>
                )}
            </div>
          ) : (
            <p class="text-xs text-gray-600 text-center py-2">Loading relationships...</p>
          )}
        </section>

        {/* Document Info */}
        <section>
          <h3 class="text-xs font-medium text-gray-400 uppercase tracking-wider mb-3">Details</h3>
          <div class="space-y-2 text-xs">
            <div class="flex justify-between">
              <span class="text-gray-500">Path</span>
              <span class="text-gray-400 font-mono truncate max-w-[150px]" title={document.path}>
                {document.path.split('/').pop()}
              </span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-500">Content Hash</span>
              <span class="text-gray-400 font-mono">
                {document.contentHash?.slice(0, 8) || 'N/A'}
              </span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-500">Embedding</span>
              <span class="text-gray-400 font-mono">
                {document.embedding ? `${document.embedding.length} dims` : 'Not generated'}
              </span>
            </div>
          </div>
        </section>
      </div>
    </aside>
  )
}
