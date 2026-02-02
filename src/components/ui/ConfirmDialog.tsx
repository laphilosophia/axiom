import * as DialogPrimitive from '@radix-ui/react-dialog'
import { AlertTriangle, X } from 'lucide-preact'

interface ConfirmDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  title: string
  description: string
  confirmLabel?: string
  cancelLabel?: string
  variant?: 'danger' | 'warning' | 'default'
  onConfirm: () => void
}

export function ConfirmDialog({
  open,
  onOpenChange,
  title,
  description,
  confirmLabel = 'Confirm',
  cancelLabel = 'Cancel',
  variant = 'default',
  onConfirm,
}: ConfirmDialogProps) {
  const variantStyles = {
    danger: 'bg-red-500 hover:bg-red-600',
    warning: 'bg-yellow-500 hover:bg-yellow-600',
    default: 'bg-accent-indigo hover:bg-accent-indigo/80',
  }

  return (
    <DialogPrimitive.Root open={open} onOpenChange={onOpenChange}>
      <DialogPrimitive.Portal>
        <DialogPrimitive.Overlay class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 animate-fade-in" />
        <DialogPrimitive.Content
          class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-50 w-full max-w-md p-6 bg-background-secondary border border-border rounded-xl shadow-2xl animate-fade-in"
        >
          <div class="flex items-start gap-4">
            {variant === 'danger' && (
              <div class="w-10 h-10 rounded-full bg-red-500/20 flex items-center justify-center flex-shrink-0">
                <AlertTriangle className="w-5 h-5 text-red-500" />
              </div>
            )}
            <div class="flex-1">
              <DialogPrimitive.Title class="text-lg font-semibold text-white">
                {title}
              </DialogPrimitive.Title>
              <DialogPrimitive.Description class="mt-2 text-sm text-gray-400">
                {description}
              </DialogPrimitive.Description>
            </div>
          </div>

          <div class="flex justify-end gap-3 mt-6">
            <DialogPrimitive.Close
              class="px-4 py-2 text-sm text-gray-400 bg-surface hover:bg-surface/80 border border-border rounded-lg transition-colors"
            >
              {cancelLabel}
            </DialogPrimitive.Close>
            <button
              onClick={() => {
                onConfirm()
                onOpenChange(false)
              }}
              class={`px-4 py-2 text-sm text-white rounded-lg transition-colors ${variantStyles[variant]}`}
            >
              {confirmLabel}
            </button>
          </div>

          <DialogPrimitive.Close
            class="absolute top-4 right-4 p-1 text-gray-500 hover:text-white
                   hover:bg-surface rounded transition-colors"
          >
            <X className="w-4 h-4" />
          </DialogPrimitive.Close>
        </DialogPrimitive.Content>
      </DialogPrimitive.Portal>
    </DialogPrimitive.Root>
  )
}
