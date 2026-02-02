import { AlertCircle, CheckCircle, Info, X, XCircle } from 'lucide-preact'
import { useEffect, useState } from 'preact/hooks'

export type ToastType = 'success' | 'error' | 'warning' | 'info'

export interface Toast {
  id: string
  message: string
  type: ToastType
  duration?: number
}

interface ToastContainerProps {
  toasts: Toast[]
  onDismiss: (id: string) => void
}

const icons = {
  success: CheckCircle,
  error: XCircle,
  warning: AlertCircle,
  info: Info,
}

const colors = {
  success: 'bg-green-500/20 border-green-500/50 text-green-400',
  error: 'bg-red-500/20 border-red-500/50 text-red-400',
  warning: 'bg-yellow-500/20 border-yellow-500/50 text-yellow-400',
  info: 'bg-blue-500/20 border-blue-500/50 text-blue-400',
}

function ToastItem({ toast, onDismiss }: { toast: Toast; onDismiss: () => void }) {
  const [isVisible, setIsVisible] = useState(false)
  const [isLeaving, setIsLeaving] = useState(false)
  const Icon = icons[toast.type]

  useEffect(() => {
    // Trigger enter animation
    requestAnimationFrame(() => setIsVisible(true))

    // Auto dismiss
    const duration = toast.duration ?? 4000
    const timer = setTimeout(() => {
      setIsLeaving(true)
      setTimeout(onDismiss, 300)
    }, duration)

    return () => clearTimeout(timer)
  }, [toast.duration, onDismiss])

  const handleDismiss = () => {
    setIsLeaving(true)
    setTimeout(onDismiss, 300)
  }

  return (
    <div
      class={`
        flex items-center gap-3 px-4 py-3 rounded-lg border backdrop-blur-md
        shadow-lg transition-all duration-300 ease-out
        ${colors[toast.type]}
        ${isVisible && !isLeaving ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-8'}
      `}>
      <Icon className="w-5 h-5 flex-shrink-0" />
      <span class="flex-1 text-sm font-medium">{toast.message}</span>
      <button
        onClick={handleDismiss}
        class="p-1 rounded hover:bg-white/10 transition-colors">
        <X className="w-4 h-4" />
      </button>
    </div>
  )
}

export function ToastContainer({ toasts, onDismiss }: ToastContainerProps) {
  if (toasts.length === 0) return null

  return (
    <div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 max-w-sm">
      {toasts.map((toast) => (
        <ToastItem key={toast.id} toast={toast} onDismiss={() => onDismiss(toast.id)} />
      ))}
    </div>
  )
}

// Toast hook for easy usage
let toastCounter = 0
let addToastCallback: ((toast: Omit<Toast, 'id'>) => void) | null = null

export function useToast() {
  const [toasts, setToasts] = useState<Toast[]>([])

  useEffect(() => {
    addToastCallback = (toast) => {
      const id = `toast-${++toastCounter}`
      setToasts((prev) => [...prev, { ...toast, id }])
    }
    return () => {
      addToastCallback = null
    }
  }, [])

  const dismiss = (id: string) => {
    setToasts((prev) => prev.filter((t) => t.id !== id))
  }

  const showToast = (message: string, type: ToastType = 'info', duration?: number) => {
    const id = `toast-${++toastCounter}`
    setToasts((prev) => [...prev, { id, message, type, duration }])
  }

  return { toasts, dismiss, showToast }
}

// Global toast function for use outside React
export function toast(message: string, type: ToastType = 'info') {
  if (addToastCallback) {
    addToastCallback({ message, type })
  }
}
