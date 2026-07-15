import * as React from "react"
import { cn } from "../shared/utils"

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, type, ...props }, ref) => {
    return (
      <input
        type={type}
        className={cn(
          "flex h-9 w-full rounded-input px-3 py-1 text-sm",
          "bg-surface dark:bg-surface-dark",
          "border border-border dark:border-border-dark",
          "text-text-primary dark:text-text-dark-primary",
          "placeholder:text-text-secondary dark:placeholder:text-text-dark-secondary",
          "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent/30 focus-visible:border-accent",
          "disabled:cursor-not-allowed disabled:opacity-50",
          "transition-colors",
          className,
        )}
        ref={ref}
        {...props}
      />
    )
  },
)
Input.displayName = "Input"

export { Input }
