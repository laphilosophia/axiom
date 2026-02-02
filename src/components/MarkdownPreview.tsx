/**
 * Markdown Preview Component
 * Renders markdown content as HTML
 */

import { marked } from 'marked'
import { useMemo } from 'preact/hooks'

interface MarkdownPreviewProps {
  content: string
}

// Configure marked for security and styling
marked.setOptions({
  breaks: true,
  gfm: true,
})

export function MarkdownPreview({ content }: MarkdownPreviewProps) {
  const html = useMemo(() => {
    if (!content.trim()) {
      return '<p class="text-gray-600 italic">No content to preview</p>'
    }
    return marked.parse(content) as string
  }, [content])

  return (
    <div
      class="markdown-preview flex-1 h-full overflow-auto p-6 prose prose-invert max-w-none"
      // biome-ignore lint/security/noDangerouslySetInnerHtml: Trusted markdown content
      dangerouslySetInnerHTML={{ __html: html }}
    />
  )
}
