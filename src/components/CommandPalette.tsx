import { FileText, Plus, Search, Settings } from 'lucide-preact'
import { useEffect, useMemo, useState } from 'preact/hooks'
import type { Document } from '../types/document'

interface CommandPaletteProps {
  isOpen: boolean
  onClose: () => void
  documents: Document[]
  onSelectDocument: (doc: Document) => void
}

type Command = {
  id: string
  title: string
  description: string
  icon: typeof FileText
  action: () => void
  shortcut?: string
}

export function CommandPalette({
  isOpen,
  onClose,
  documents,
  onSelectDocument,
}: CommandPaletteProps) {
  const [query, setQuery] = useState('')
  const [selectedIndex, setSelectedIndex] = useState(0)

  const commands: Command[] = useMemo(() => {
    const docs = documents
      .filter(
        (d) =>
          d.title.toLowerCase().includes(query.toLowerCase()) ||
          d.tags.some((t) => t.toLowerCase().includes(query.toLowerCase())),
      )
      .slice(0, 5)
      .map((d) => ({
        id: `doc-${d.id}`,
        title: d.title,
        description: `Open document • ${d.status} • ${new Date(d.updatedAt).toLocaleDateString()}`,
        icon: FileText,
        action: () => onSelectDocument(d),
      }))

    const actions: Command[] = [
      {
        id: 'new-doc',
        title: 'Create New Document',
        description: 'Start writing a new document',
        icon: Plus,
        action: () => {
          // TODO: Create new document
          onClose()
        },
        shortcut: '⌘N',
      },
      {
        id: 'settings',
        title: 'Open Settings',
        description: 'Configure application preferences',
        icon: Settings,
        action: () => {
          // TODO: Open settings
          onClose()
        },
        shortcut: '⌘,',
      },
    ]

    return [...docs, ...actions]
  }, [documents, query, onSelectDocument, onClose])

  useEffect(() => {
    setSelectedIndex(0)
  }, [query])

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isOpen) return

      switch (e.key) {
        case 'Escape':
          onClose()
          break
        case 'ArrowDown':
          e.preventDefault()
          setSelectedIndex((i) => Math.min(i + 1, commands.length - 1))
          break
        case 'ArrowUp':
          e.preventDefault()
          setSelectedIndex((i) => Math.max(i - 1, 0))
          break
        case 'Enter':
          e.preventDefault()
          commands[selectedIndex]?.action()
          onClose()
          break
      }
    }

    document.addEventListener('keydown', handleKeyDown)
    return () => document.removeEventListener('keydown', handleKeyDown)
  }, [isOpen, commands, selectedIndex, onClose])

  useEffect(() => {
    if (isOpen) {
      setQuery('')
      setSelectedIndex(0)
    }
  }, [isOpen])

  if (!isOpen) return null

  return (
    <div class="fixed inset-0 z-50 flex items-start justify-center pt-[20vh]">
      {/* Backdrop */}
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" onClick={onClose} />

      {/* Palette */}
      <div class="relative w-full max-w-xl bg-background-secondary border border-border rounded-xl shadow-2xl overflow-hidden animate-fade-in">
        {/* Search Input */}
        <div class="flex items-center px-4 py-4 border-b border-border">
          <Search className="w-5 h-5 text-gray-400" />
          <input
            type="text"
            value={query}
            onInput={(e) => setQuery((e.target as HTMLInputElement).value)}
            placeholder="Search documents or run commands..."
            class="flex-1 ml-3 bg-transparent border-none outline-none text-white
                   placeholder-gray-500 text-lg"
            autoFocus
          />
          <kbd class="px-2 py-1 text-xs bg-surface rounded text-gray-400">ESC</kbd>
        </div>

        {/* Results */}
        <div class="max-h-[50vh] overflow-y-auto scrollbar-thin py-2">
          {commands.length > 0 ? (
            commands.map((command, index) => {
              const Icon = command.icon
              return (
                <button
                  key={command.id}
                  onClick={() => {
                    command.action()
                    onClose()
                  }}
                  class={`w-full px-4 py-3 flex items-center gap-3 text-left transition-all ${
                    index === selectedIndex ? 'bg-accent-indigo/20' : 'hover:bg-surface'
                  }`}>
                  <div
                    class={`w-8 h-8 rounded-lg flex items-center justify-center ${
                      index === selectedIndex ? 'bg-accent-indigo/30' : 'bg-surface'
                    }`}>
                    <Icon
                      className={`w-4 h-4 ${
                        index === selectedIndex ? 'text-accent-indigo' : 'text-gray-400'
                      }`}
                    />
                  </div>

                  <div class="flex-1 min-w-0">
                    <h4
                      class={`text-sm font-medium truncate ${
                        index === selectedIndex ? 'text-white' : 'text-gray-200'
                      }`}>
                      {command.title}
                    </h4>
                    <p class="text-xs text-gray-500 truncate">{command.description}</p>
                  </div>

                  {command.shortcut && (
                    <kbd class="px-2 py-1 text-xs bg-surface rounded text-gray-400">
                      {command.shortcut}
                    </kbd>
                  )}
                </button>
              )
            })
          ) : (
            <div class="px-4 py-8 text-center">
              <p class="text-sm text-gray-500">No results found</p>
              <p class="text-xs text-gray-600 mt-1">Try a different search term</p>
            </div>
          )}
        </div>

        {/* Footer */}
        <div class="px-4 py-2 bg-surface border-t border-border flex items-center justify-between text-xs text-gray-500">
          <div class="flex items-center gap-4">
            <span class="flex items-center gap-1">
              <kbd class="px-1.5 py-0.5 bg-background rounded">↑↓</kbd>
              Navigate
            </span>
            <span class="flex items-center gap-1">
              <kbd class="px-1.5 py-0.5 bg-background rounded">↵</kbd>
              Select
            </span>
          </div>
          <span>{commands.length} results</span>
        </div>
      </div>
    </div>
  )
}
