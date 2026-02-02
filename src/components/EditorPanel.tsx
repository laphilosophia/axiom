import { invoke } from '@tauri-apps/api/tauri'
import type { Document, DocumentStatus } from '@types/document'
import { debounce } from '@utils/debounce'
import { AlertCircle, Archive, FileCheck, Save } from 'lucide-react'
import { useCallback, useEffect, useState } from 'preact/hooks'

interface EditorPanelProps {
  document: Document | null
  onDocumentChange: () => void
}

const statusOptions: { value: DocumentStatus; label: string; icon: typeof Save }[] = [
  { value: 'draft', label: 'Draft', icon: AlertCircle },
  { value: 'active', label: 'Active', icon: FileCheck },
  { value: 'superseded', label: 'Superseded', icon: Archive },
  { value: 'archived', label: 'Archived', icon: Archive },
]

export function EditorPanel({ document, onDocumentChange }: EditorPanelProps) {
  const [title, setTitle] = useState('')
  const [content, setContent] = useState('')
  const [tags, setTags] = useState('')
  const [isSaving, setIsSaving] = useState(false)
  const [lastSaved, setLastSaved] = useState<Date | null>(null)
  const [error, setError] = useState<string | null>(null)

  // Load document data
  useEffect(() => {
    if (document) {
      setTitle(document.title)
      setContent(document.content)
      setTags(document.tags.join(', '))
      setError(null)
    } else {
      setTitle('')
      setContent('')
      setTags('')
    }
  }, [document?.id])

  // Auto-save with debounce
  const saveDocument = useCallback(
    debounce(async (updates: Partial<Document>) => {
      if (!document) return

      setIsSaving(true)
      setError(null)

      try {
        await invoke('update_document', {
          id: document.id,
          ...updates,
        })
        setLastSaved(new Date())
        onDocumentChange()
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to save')
      } finally {
        setIsSaving(false)
      }
    }, 1000),
    [document, onDocumentChange],
  )

  const handleTitleChange = (newTitle: string) => {
    setTitle(newTitle)
    saveDocument({ title: newTitle })
  }

  const handleContentChange = (newContent: string) => {
    setContent(newContent)
    saveDocument({ content: newContent })
  }

  const handleTagsChange = (newTags: string) => {
    setTags(newTags)
    const tagArray = newTags
      .split(',')
      .map((t) => t.trim())
      .filter(Boolean)
    saveDocument({ tags: tagArray })
  }

  const handleStatusChange = async (newStatus: DocumentStatus) => {
    if (!document) return

    try {
      await invoke('update_document_status', {
        id: document.id,
        status: newStatus,
      })
      onDocumentChange()
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update status')
    }
  }

  if (!document) {
    return (
      <main class="flex-1 h-full flex items-center justify-center bg-background">
        <div class="text-center">
          <div class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-surface flex items-center justify-center">
            <Save className="w-8 h-8 text-gray-600" />
          </div>
          <h2 class="text-xl font-semibold text-gray-400">No Document Selected</h2>
          <p class="text-sm text-gray-600 mt-2">
            Select a document from the library or create a new one
          </p>
        </div>
      </main>
    )
  }

  const isReadOnly = document.status === 'superseded' || document.status === 'archived'

  return (
    <main class="flex-1 h-full flex flex-col bg-background">
      {/* Editor Header */}
      <header class="px-6 py-4 border-b border-border">
        <div class="flex items-center justify-between">
          <div class="flex-1">
            <input
              type="text"
              value={title}
              onInput={(e) => handleTitleChange((e.target as HTMLInputElement).value)}
              disabled={isReadOnly}
              placeholder="Document Title"
              class="w-full text-2xl font-semibold bg-transparent border-none outline-none
                     text-white placeholder-gray-600 focus:ring-0 disabled:opacity-50"
            />
          </div>

          <div class="flex items-center gap-3">
            {/* Status Selector */}
            <select
              value={document.status}
              onChange={(e) =>
                handleStatusChange((e.target as HTMLSelectElement).value as DocumentStatus)
              }
              disabled={isReadOnly}
              class="px-3 py-1.5 bg-surface border border-border rounded-lg text-sm
                     text-white focus:border-accent-indigo focus:outline-none
                     disabled:opacity-50 cursor-pointer">
              {statusOptions.map((opt) => (
                <option key={opt.value} value={opt.value}>
                  {opt.label}
                </option>
              ))}
            </select>

            {/* Save Indicator */}
            {isSaving ? (
              <span class="text-xs text-gray-500 flex items-center gap-1">
                <span class="w-3 h-3 border border-gray-500 border-t-transparent rounded-full animate-spin" />
                Saving...
              </span>
            ) : lastSaved ? (
              <span class="text-xs text-gray-600 flex items-center gap-1">
                <Save className="w-3 h-3" />
                Saved {lastSaved.toLocaleTimeString()}
              </span>
            ) : null}
          </div>
        </div>

        {/* Tags Input */}
        <div class="mt-3 flex items-center gap-2">
          <input
            type="text"
            value={tags}
            onInput={(e) => handleTagsChange((e.target as HTMLInputElement).value)}
            disabled={isReadOnly}
            placeholder="Tags (comma separated)"
            class="flex-1 px-3 py-1.5 bg-surface border border-border rounded-lg text-sm
                   text-white placeholder-gray-600 focus:border-accent-indigo
                   focus:outline-none disabled:opacity-50"
          />
        </div>

        {/* Error Message */}
        {error && (
          <div class="mt-3 flex items-center gap-2 text-sm text-red-400">
            <AlertCircle className="w-4 h-4" />
            {error}
          </div>
        )}

        {/* Read-only Banner */}
        {isReadOnly && (
          <div
            class="mt-3 px-4 py-2 bg-status-superseded/10 border border-status-superseded/30
                      rounded-lg flex items-center gap-2">
            <Archive className="w-4 h-4 text-status-superseded" />
            <span class="text-sm text-status-superseded">
              This document is {document.status}. Editing is disabled.
            </span>
          </div>
        )}
      </header>

      {/* Editor Content */}
      <div class="flex-1 overflow-hidden">
        <textarea
          value={content}
          onInput={(e) => handleContentChange((e.target as HTMLTextAreaElement).value)}
          disabled={isReadOnly}
          placeholder="Start writing..."
          class="w-full h-full p-6 bg-transparent resize-none outline-none text-gray-200
                 placeholder-gray-600 font-mono text-sm leading-relaxed
                 disabled:opacity-70 disabled:sepia-[.3]"
          spellcheck={false}
        />
      </div>

      {/* Footer Info */}
      <footer class="px-6 py-2 border-t border-border text-xs text-gray-600 flex items-center justify-between">
        <div class="flex items-center gap-4">
          <span>
            ID: <code class="font-mono text-gray-500">{document.id.slice(0, 8)}...</code>
          </span>
          <span>Created: {new Date(document.createdAt).toLocaleDateString()}</span>
          <span>Updated: {new Date(document.updatedAt).toLocaleDateString()}</span>
        </div>
        <div class="flex items-center gap-4">
          <span>{content.length} characters</span>
          <span>{content.split(/\s+/).filter(Boolean).length} words</span>
          <span>{content.split('\n').filter(Boolean).length} lines</span>
        </div>
      </footer>
    </main>
  )
}
