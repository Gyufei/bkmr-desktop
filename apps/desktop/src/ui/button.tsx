import * as React from "react"
import { cn } from "../shared/utils"

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link"
  size?: "default" | "sm" | "lg" | "icon" | "icon-sm"
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = "default", size = "default", ...props }, ref) => {
    return (
      <button
        className={cn(
          "inline-flex items-center justify-center whitespace-nowrap rounded-btn text-sm font-medium transition-colors",
          "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent/40",
          "disabled:pointer-events-none disabled:opacity-50",
          {
            "bg-accent text-white hover:bg-accent-hover dark:bg-accent-dark dark:text-white": variant === "default",
            "bg-danger text-white hover:opacity-90 dark:bg-danger-dark": variant === "destructive",
            "border border-border dark:border-border-dark bg-transparent hover:bg-accent-bg dark:hover:bg-accent-dark-bg text-text-primary dark:text-text-dark-primary": variant === "outline",
            "bg-surface-sidebar dark:bg-surface-dark-sidebar text-text-primary dark:text-text-dark-primary hover:opacity-80": variant === "secondary",
            "bg-transparent hover:bg-accent-bg dark:hover:bg-accent-dark-bg text-text-primary dark:text-text-dark-primary": variant === "ghost",
            "text-accent dark:text-accent-dark underline-offset-4 hover:underline": variant === "link",
          },
          {
            "h-9 px-4 py-2": size === "default",
            "h-8 rounded-btn px-3 text-xs": size === "sm",
            "h-10 rounded-btn px-8": size === "lg",
            "h-9 w-9": size === "icon",
            "h-7 w-7": size === "icon-sm",
          },
          className,
        )}
        ref={ref}
        {...props}
      />
    )
  },
)
Button.displayName = "Button"

export { Button }
