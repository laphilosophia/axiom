import * as SelectPrimitive from '@radix-ui/react-select'
import { Check, ChevronDown, ChevronUp } from 'lucide-preact'

interface SelectProps {
  value: string
  onValueChange: (value: string) => void
  options: { value: string; label: string }[]
  placeholder?: string
  disabled?: boolean
}

export function Select({
  value,
  onValueChange,
  options,
  placeholder = 'Select...',
  disabled = false,
}: SelectProps) {
  return (
    <SelectPrimitive.Root value={value} onValueChange={onValueChange} disabled={disabled}>
      <SelectPrimitive.Trigger
        class="inline-flex items-center justify-between gap-2 px-3 py-1.5 min-w-[120px]
               bg-surface border border-border rounded-lg text-sm text-white
               hover:border-accent-indigo/50 focus:border-accent-indigo focus:outline-none
               disabled:opacity-50 disabled:cursor-not-allowed
               data-[placeholder]:text-gray-500"
      >
        <SelectPrimitive.Value placeholder={placeholder} />
        <SelectPrimitive.Icon>
          <ChevronDown className="w-4 h-4 text-gray-400" />
        </SelectPrimitive.Icon>
      </SelectPrimitive.Trigger>

      <SelectPrimitive.Portal>
        <SelectPrimitive.Content
          class="overflow-hidden bg-background-secondary border border-border rounded-lg
                 shadow-xl animate-fade-in z-50"
          position="popper"
          sideOffset={4}
        >
          <SelectPrimitive.ScrollUpButton class="flex items-center justify-center h-6 bg-background-secondary cursor-default">
            <ChevronUp className="w-4 h-4 text-gray-400" />
          </SelectPrimitive.ScrollUpButton>

          <SelectPrimitive.Viewport class="p-1">
            {options.map((option) => (
              <SelectPrimitive.Item
                key={option.value}
                value={option.value}
                class="relative flex items-center px-8 py-2 text-sm text-white rounded-md
                       cursor-pointer select-none
                       hover:bg-accent-indigo/20 focus:bg-accent-indigo/20 focus:outline-none
                       data-[highlighted]:bg-accent-indigo/20
                       data-[disabled]:text-gray-500 data-[disabled]:pointer-events-none"
              >
                <SelectPrimitive.ItemText>{option.label}</SelectPrimitive.ItemText>
                <SelectPrimitive.ItemIndicator class="absolute left-2">
                  <Check className="w-4 h-4 text-accent-indigo" />
                </SelectPrimitive.ItemIndicator>
              </SelectPrimitive.Item>
            ))}
          </SelectPrimitive.Viewport>

          <SelectPrimitive.ScrollDownButton class="flex items-center justify-center h-6 bg-background-secondary cursor-default">
            <ChevronDown className="w-4 h-4 text-gray-400" />
          </SelectPrimitive.ScrollDownButton>
        </SelectPrimitive.Content>
      </SelectPrimitive.Portal>
    </SelectPrimitive.Root>
  )
}
