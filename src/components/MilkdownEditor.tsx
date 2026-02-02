/**
 * CodeMirror 6 Markdown Editor
 *
 * Features:
 * - Syntax highlighting for markdown
 * - Dark theme (One Dark)
 * - Keyboard shortcuts
 * - Read-only mode support
 */

import { defaultKeymap, history, historyKeymap } from '@codemirror/commands'
import { markdown, markdownLanguage } from '@codemirror/lang-markdown'
import { languages } from '@codemirror/language-data'
import { EditorState } from '@codemirror/state'
import { oneDark } from '@codemirror/theme-one-dark'
import { EditorView, keymap, placeholder as placeholderExt } from '@codemirror/view'
import { useEffect, useRef } from 'preact/hooks'

interface CodeMirrorEditorProps {
  content: string
  onChange: (content: string) => void
  readOnly?: boolean
  placeholder?: string
}

// Custom dark theme to match Axiom's Monolith design
const axiomTheme = EditorView.theme({
  '&': {
    height: '100%',
    fontSize: '14px',
    backgroundColor: 'transparent',
  },
  '.cm-content': {
    fontFamily: "'JetBrains Mono', 'Fira Code', monospace",
    padding: '1.5rem',
    caretColor: '#818cf8',
  },
  '.cm-cursor': {
    borderLeftColor: '#818cf8',
  },
  '.cm-selectionBackground, &.cm-focused .cm-selectionBackground': {
    backgroundColor: 'rgba(99, 102, 241, 0.5) !important',
  },
  '&.cm-focused .cm-selectionBackground, .cm-selectionBackground, ::selection': {
    backgroundColor: 'rgba(99, 102, 241, 0.5) !important',
  },
  '.cm-gutters': {
    backgroundColor: 'transparent',
    border: 'none',
    color: '#4b5563',
  },
  '.cm-activeLineGutter': {
    backgroundColor: 'transparent',
  },
  '.cm-activeLine': {
    backgroundColor: 'rgba(255, 255, 255, 0.03)',
  },
  '.cm-scroller': {
    overflow: 'auto',
  },
  // Markdown-specific styling
  '.cm-header-1': { fontSize: '1.5em', fontWeight: 'bold', color: '#7ebed6ff' },
  '.cm-header-2': { fontSize: '1.3em', fontWeight: 'bold', color: '#7ebed6ff' },
  '.cm-header-3': { fontSize: '1.1em', fontWeight: 'bold', color: '#7ebed6ff' },
  '.cm-strong': { fontWeight: 'bold', color: '#fbbf24' },
  '.cm-emphasis': { fontStyle: 'italic', color: '#a5f3fc' },
  '.cm-strikethrough': { textDecoration: 'line-through' },
  '.cm-link': { color: '#60a5fa', textDecoration: 'underline' },
  '.cm-url': { color: '#6b7280' },
  '.cm-code': {
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: '3px',
    padding: '2px 4px',
    color: '#7ebed6ff',
  },
}, { dark: true })

export function CodeMirrorEditor({
  content,
  onChange,
  readOnly = false,
  placeholder = 'Start writing...',
}: CodeMirrorEditorProps) {
  const editorRef = useRef<HTMLDivElement>(null)
  const viewRef = useRef<EditorView | null>(null)
  const onChangeRef = useRef(onChange)

  // Keep onChange ref updated
  useEffect(() => {
    onChangeRef.current = onChange
  }, [onChange])

  useEffect(() => {
    if (!editorRef.current) return

    const updateListener = EditorView.updateListener.of((update) => {
      if (update.docChanged) {
        onChangeRef.current(update.state.doc.toString())
      }
    })

    const state = EditorState.create({
      doc: content,
      extensions: [
        axiomTheme,
        oneDark,
        history(),
        keymap.of([...defaultKeymap, ...historyKeymap]),
        markdown({ base: markdownLanguage, codeLanguages: languages }),
        placeholderExt(placeholder),
        updateListener,
        EditorView.lineWrapping,
        EditorState.readOnly.of(readOnly),
      ],
    })

    const view = new EditorView({
      state,
      parent: editorRef.current,
    })

    viewRef.current = view

    return () => {
      view.destroy()
      viewRef.current = null
    }
  }, [])

  // Sync external content changes to editor
  useEffect(() => {
    if (viewRef.current) {
      const currentContent = viewRef.current.state.doc.toString()
      if (content !== currentContent) {
        viewRef.current.dispatch({
          changes: {
            from: 0,
            to: currentContent.length,
            insert: content,
          },
        })
      }
    }
  }, [content])

  return (
    <div
      ref={editorRef}
      class={`codemirror-container flex-1 h-full overflow-hidden ${readOnly ? 'opacity-70' : ''}`}
    />
  )
}

// Re-export as MilkdownEditor for backward compatibility
export { CodeMirrorEditor as MilkdownEditor }
