import { FileText, Filter, Plus, Search, Trash2 } from 'lucide-preact'
import { useCallback, useMemo, useState } from 'preact/hooks'
import type { Document, DocumentStatus } from '../types/document'

interface LibraryPanelProps {
  documents: Document[]
  selectedDocument: Document | null
  onSelectDocument: (doc: Document) => void
  onCreateDocument: () => void
  onDeleteDocument: (id: string) => void
}

const statusOrder: DocumentStatus[] = ['active', 'draft', 'superseded', 'archived']

const statusLabels: Record<DocumentStatus, string> = {
  active: 'Active',
  draft: 'Draft',
  superseded: 'Superseded',
  archived: 'Archived',
}

export function LibraryPanel({
  documents,
  selectedDocument,
  onSelectDocument,
  onCreateDocument,
  onDeleteDocument,
}: LibraryPanelProps) {
  const [searchQuery, setSearchQuery] = useState('')
  const [statusFilter, setStatusFilter] = useState<DocumentStatus | null>(null)

  const groupedDocuments = useMemo(() => {
    let filtered = documents

    if (searchQuery) {
      const query = searchQuery.toLowerCase()
      filtered = filtered.filter(
        (d) =>
          d.title.toLowerCase().includes(query) ||
          d.tags.some((t) => t.toLowerCase().includes(query)),
      )
    }

    if (statusFilter) {
      filtered = filtered.filter((d) => d.status === statusFilter)
    }

    const grouped = new Map<DocumentStatus, Document[]>()
    statusOrder.forEach((status) => {
      const docs = filtered.filter((d) => d.status === status)
      if (docs.length > 0) {
        grouped.set(
          status,
          docs.sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()),
        )
      }
    })

    return grouped
  }, [documents, searchQuery, statusFilter])

  const handleDeleteDocument = useCallback(async (e: MouseEvent, id: string) => {
    e.stopPropagation();
    e.preventDefault();
    const confirmed = await window.confirm('Delete this document?');
    if (confirmed) {
      onDeleteDocument(id)
    }
  }, [onDeleteDocument])

  return (
    <aside class="w-80 h-full glass-panel flex flex-col border-r border-border">
      {/* Header */}
      <div class="p-4 border-b border-border">
        <div class="flex items-center justify-between mb-4">
          <h1 class="text-lg font-semibold text-white flex items-center gap-2">
            <FileText className="w-5 h-5 text-accent-indigo" />
            Axiom
          </h1>
          <button onClick={onCreateDocument} class="btn-primary p-2" title="New Document">
            <Plus className="w-4 h-4" />
          </button>
        </div>

        {/* Search */}
        <div class="relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
          <input
            type="text"
            placeholder="Search documents..."
            value={searchQuery}
            onInput={(e) => setSearchQuery((e.target as HTMLInputElement).value)}
            class="w-full pl-9 pr-4 py-2 bg-background-secondary rounded-lg text-sm text-white
                   placeholder-gray-500 border border-border focus:border-accent-indigo
                   focus:outline-none transition-colors"
          />
        </div>

        {/* Status Filter */}
        <div class="flex items-center gap-2 mt-3">
          <Filter className="w-3 h-3 text-gray-400" />
          <div class="flex gap-1">
            {(['active', 'draft', 'superseded', 'archived'] as DocumentStatus[]).map((status) => (
              <button
                key={status}
                onClick={() => setStatusFilter(statusFilter === status ? null : status)}
                class={`px-2 py-1 text-xs rounded-md transition-all ${
                  statusFilter === status
                    ? 'bg-accent-indigo text-white'
                    : 'bg-surface text-gray-400 hover:text-white'
                }`}>
                {statusLabels[status]}
              </button>
            ))}
          </div>
        </div>
      </div>

      {/* Document List */}
      <div class="flex-1 overflow-y-auto scrollbar-thin p-2">
        {Array.from(groupedDocuments.entries()).map(([status, docs]) => (
          <div key={status} class="mb-4">
            <h3 class="px-2 py-1 text-xs font-medium text-gray-500 uppercase tracking-wider flex items-center gap-2">
              <span class={`status-indicator ${status}`} />
              {statusLabels[status]}
              <span class="text-gray-600">({docs.length})</span>
            </h3>

            <div class="mt-1 space-y-1">
              {docs.map((doc) => (
                <div
                  key={doc.id}
                  class={`flex items-center gap-1 rounded-lg transition-all duration-200 group ${
                    selectedDocument?.id === doc.id
                      ? 'bg-accent-indigo/20 border border-accent-indigo/50'
                      : 'hover:bg-surface border border-transparent'
                  }`}>
                  <button
                    onClick={() => onSelectDocument(doc)}
                    class="flex-1 text-left p-3">
                    <h4
                      class={`text-sm font-medium truncate ${
                        status === 'superseded' ? 'text-gray-500 line-through' : 'text-white'
                      }`}>
                      {doc.title}
                    </h4>
                    <div class="flex items-center justify-between mt-1">
                      <span class="text-xs text-gray-500">
                        {new Date(doc.updatedAt).toLocaleDateString()}
                      </span>
                      {doc.tags.length > 0 && (
                        <span class="text-xs text-gray-600">
                          {doc.tags.slice(0, 2).join(', ')}
                          {doc.tags.length > 2 && '...'}
                        </span>
                      )}
                    </div>
                  </button>
                  <button
                    type="button"
                    onClick={(e) => handleDeleteDocument(e, doc.id)}
                    class={`p-2 mr-1 text-gray-500 hover:text-red-500 hover:bg-red-500/10 rounded transition-all ${
                      selectedDocument?.id === doc.id ? 'opacity-100' : 'opacity-0 group-hover:opacity-100'
                    }`}
                    title="Delete document">
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              ))}
            </div>
          </div>
        ))}

        {groupedDocuments.size === 0 && (
          <div class="text-center py-8 text-gray-500">
            <p class="text-sm">No documents found</p>
            <p class="text-xs mt-1">Create a new document to get started</p>
          </div>
        )}
      </div>

      {/* Footer */}
      <div class="p-3 border-t border-border text-xs text-gray-600 flex items-center justify-between">
        <span>{documents.length} documents</span>
        <span class="flex items-center gap-1">
          <span class="w-2 h-2 rounded-full bg-status-active animate-pulse" />
          Ready
        </span>
      </div>
    </aside>
  )
}
