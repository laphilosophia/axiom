import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'preact/hooks'
import { CommandPalette } from './components/CommandPalette'
import { EditorPanel } from './components/EditorPanel'
import { LibraryPanel } from './components/LibraryPanel'
import { OrchestrationPanel } from './components/OrchestrationPanel'
import { ToastContainer, useToast } from './components/Toast'
import { documentsSignal, useAppStore } from './stores/appStore'
import type { Document } from './types/document'

export function App() {
  const [isCommandPaletteOpen, setIsCommandPaletteOpen] = useState(false)
  const [isWorkspaceReady, setIsWorkspaceReady] = useState(false)
  const [showWorkspaceDialog, setShowWorkspaceDialog] = useState(false)
  const {
    documents,
    selectedDocument,
    isLoading,
    setDocuments,
    setSelectedDocument,
    setIsLoading,
  } = useAppStore()
  const { toasts, dismiss, showToast } = useToast()

  useEffect(() => {
    // Check workspace on startup
    initializeApp()

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

  const initializeApp = async () => {
    setIsLoading(true)
    try {
      // Check if workspace exists
      const workspacePath = await invoke<string | null>('get_workspace_path')

      if (workspacePath) {
        // Try to load documents - if it fails, we need to initialize
        try {
          const docs = await invoke<Document[]>('get_documents')
          setDocuments(docs)
          setIsWorkspaceReady(true)
          setIsLoading(false)
          return
        } catch (loadError) {
          console.log('Workspace path exists but not initialized, setting up...')
          // Fall through to setup
        }
      }

      // Need to setup workspace
      await setupDefaultWorkspace()
    } catch (error) {
      console.error('App initialization failed:', error)
      setShowWorkspaceDialog(true)
      setIsLoading(false)
    }
  }

  const setupDefaultWorkspace = async () => {
    try {
      // Use absolute home directory path
      const homeDir = await invoke<string>('get_home_dir')
      const defaultPath = `${homeDir}/AxiomDocuments`
      await invoke('set_workspace_path', { path: defaultPath })

      // Now try to load documents
      const docs = await invoke<Document[]>('get_documents')
      setDocuments(docs)
      setIsWorkspaceReady(true)
      setShowWorkspaceDialog(false)
    } catch (error) {
      console.error('Failed to setup default workspace:', error)
      setShowWorkspaceDialog(true)
    } finally {
      setIsLoading(false)
    }
  }

  const handleSelectWorkspace = async () => {
    // Use a simple default for now
    await setupDefaultWorkspace()
  }

  const handleSelectDocument = (doc: Document) => {
    setSelectedDocument(doc)
  }

  const handleCreateDocument = async () => {
    if (!isWorkspaceReady) {
      showToast('Please wait for workspace to be initialized...', 'warning')
      return
    }

    try {
      const newDoc = await invoke<Document>('create_document', {
        title: 'Untitled Document',
        content: '',
      })
      const currentDocs = documentsSignal.value
      setDocuments([newDoc, ...currentDocs])
      setSelectedDocument(newDoc)
      showToast('Document created', 'success')
    } catch (error) {
      console.error('Failed to create document:', error)
      showToast('Failed to create document', 'error')
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

  // Workspace Selection Dialog
  if (showWorkspaceDialog) {
    return (
      <div class="h-screen w-screen bg-background flex items-center justify-center">
        <div class="glass-panel p-8 rounded-xl max-w-md w-full mx-4">
          <div class="text-center">
            <div class="w-16 h-16 bg-accent-indigo/20 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg
                class="w-8 h-8 text-accent-indigo"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24">
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
                />
              </svg>
            </div>
            <h2 class="text-xl font-semibold text-white mb-2">Welcome to Axiom</h2>
            <p class="text-gray-400 text-sm mb-6">
              Select a folder where your documents will be stored.
            </p>
            <button
              onClick={handleSelectWorkspace}
              class="w-full btn-primary py-3 rounded-lg font-medium">
              Set Default Workspace
            </button>
          </div>
        </div>
      </div>
    )
  }

  // Close current document
  const handleCloseDocument = () => {
    setSelectedDocument(null)
  }

  // Delete a document
  const handleDeleteDocument = async (id: string) => {
    try {
      await invoke('delete_document', { id })
      if (selectedDocument?.id === id) {
        setSelectedDocument(null)
      }
      await refreshDocuments()
      showToast('Document deleted successfully', 'success')
    } catch (error) {
      console.error('Failed to delete document:', error)
      showToast('Failed to delete document', 'error')
    }
  }

  // Refresh documents list from backend
  const refreshDocuments = async () => {
    try {
      const docs = await invoke<Document[]>('get_documents')
      setDocuments(docs)
      // Update selected document if it was modified
      if (selectedDocument) {
        const updated = docs.find((d) => d.id === selectedDocument.id)
        if (updated) {
          setSelectedDocument(updated)
        }
      }
    } catch (error) {
      console.error('Failed to refresh documents:', error)
    }
  }

  return (
    <div class="h-screen w-screen bg-background flex overflow-hidden">
      {/* Left Panel - Library */}
      <LibraryPanel
        documents={documents}
        selectedDocument={selectedDocument}
        onSelectDocument={handleSelectDocument}
        onCreateDocument={handleCreateDocument}
        onDeleteDocument={handleDeleteDocument}
      />

      {/* Center Panel - Editor */}
      <EditorPanel
        document={selectedDocument}
        onDocumentChange={refreshDocuments}
        onClose={handleCloseDocument}
      />

      {/* Right Panel - Orchestration */}
      <OrchestrationPanel document={selectedDocument} onDocumentChange={refreshDocuments} />

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

      {/* Toast Notifications */}
      <ToastContainer toasts={toasts} onDismiss={dismiss} />
    </div>
  )
}
