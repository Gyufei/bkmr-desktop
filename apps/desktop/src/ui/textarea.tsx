import * as React from "react"
import { cn } from "../shared/utils"

export interface TextareaProps
  extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {}

const Textarea = React.forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ className, ...props }, ref) => {
    return (
      <textarea
        className={cn(
          "flex min-h-[60px] w-full rounded-input border px-3 py-2 text-sm",
          "bg-surface dark:bg-surface-dark",
          "border-border dark:border-border-dark",
          "text-text-primary dark:text-text-dark-primary",
          "placeholder:text-text-secondary dark:placeholder:text-text-dark-secondary",
          "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent/30 focus-visible:border-accent",
          "disabled:cursor-not-allowed disabled:opacity-50",
          "resize-none transition-colors",
          className,
        )}
        ref={ref}
        {...props}
      />
    )
  },
)
Textarea.displayName = "Textarea"

export { Textarea }
