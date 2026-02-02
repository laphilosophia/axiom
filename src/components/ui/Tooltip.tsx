import * as TooltipPrimitive from '@radix-ui/react-tooltip'

interface TooltipProps {
  children: preact.ComponentChildren
  content: string
  side?: 'top' | 'right' | 'bottom' | 'left'
  delayDuration?: number
}

export function Tooltip({
  children,
  content,
  side = 'top',
  delayDuration = 300,
}: TooltipProps) {
  return (
    <TooltipPrimitive.Provider delayDuration={delayDuration}>
      <TooltipPrimitive.Root>
        <TooltipPrimitive.Trigger asChild>
          {children}
        </TooltipPrimitive.Trigger>
        <TooltipPrimitive.Portal>
          <TooltipPrimitive.Content
            side={side}
            sideOffset={4}
            class="px-3 py-1.5 text-sm text-white bg-background-secondary border border-border
                   rounded-lg shadow-xl animate-fade-in z-50"
          >
            {content}
            <TooltipPrimitive.Arrow class="fill-background-secondary" />
          </TooltipPrimitive.Content>
        </TooltipPrimitive.Portal>
      </TooltipPrimitive.Root>
    </TooltipPrimitive.Provider>
  )
}
