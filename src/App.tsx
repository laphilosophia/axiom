import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'preact/hooks'
import { CommandPalette } from './components/CommandPalette'
import { EditorPanel } from './components/EditorPanel'
import { LibraryPanel } from './components/LibraryPanel'
import { OrchestrationPanel } from './components/OrchestrationPanel'
import { useAppStore } from './stores/appStore'
import type { Document } from './types/document'

export function App() {
  const [isCommandPaletteOpen, setIsCommandPaletteOpen] = useState(false)
  const {
    documents,
    selectedDocument,
    isLoading,
    setDocuments,
    setSelectedDocument,
    setIsLoading,
  } = useAppStore()

  useEffect(() => {
    // Initial load
    loadDocuments()

    // Command palette shortcut
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault()
        setIsCommandPaletteOpen(true)
      }
    }

    document.addEventListener('keydown', handleKeyDown)
    return () => document.removeEventListener('keydown', handleKeyDown)
  }, [])

  const loadDocuments = async () => {
    setIsLoading(true)
    try {
      const docs = await invoke<Document[]>('get_documents')
      setDocuments(docs)
    } catch (error) {
      console.error('Failed to load documents:', error)
    } finally {
      setIsLoading(false)
    }
  }

  const handleSelectDocument = (doc: Document) => {
    setSelectedDocument(doc)
  }

  const handleCreateDocument = async () => {
    try {
      const newDoc = await invoke<Document>('create_document', {
        title: 'Untitled Document',
        content: '',
      })
      setDocuments([newDoc, ...documents])
      setSelectedDocument(newDoc)
    } catch (error) {
      console.error('Failed to create document:', error)
    }
  }

  if (isLoading) {
    return (
      <div class="h-screen w-screen flex items-center justify-center bg-background">
        <div class="flex flex-col items-center gap-4">
          <div class="w-8 h-8 border-2 border-accent-indigo border-t-transparent rounded-full animate-spin" />
          <p class="text-gray-400 text-sm">Loading Axiom...</p>
        </div>
      </div>
    )
  }

  return (
    <div class="h-screen w-screen bg-background flex overflow-hidden">
      {/* Left Panel - Library */}
      <LibraryPanel
        documents={documents}
        selectedDocument={selectedDocument}
        onSelectDocument={handleSelectDocument}
        onCreateDocument={handleCreateDocument}
      />

      {/* Center Panel - Editor */}
      <EditorPanel document={selectedDocument} onDocumentChange={loadDocuments} />

      {/* Right Panel - Orchestration */}
      <OrchestrationPanel document={selectedDocument} onDocumentChange={loadDocuments} />

      {/* Command Palette */}
      <CommandPalette
        isOpen={isCommandPaletteOpen}
        onClose={() => setIsCommandPaletteOpen(false)}
        documents={documents}
        onSelectDocument={(doc) => {
          handleSelectDocument(doc)
          setIsCommandPaletteOpen(false)
        }}
      />
    </div>
  )
}
