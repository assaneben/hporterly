import type { ButtonHTMLAttributes } from "react";

import { cx } from "@/lib/utils";

type ButtonVariant = "primary" | "secondary" | "danger" | "ghost" | "porter";

type Props = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: ButtonVariant;
  fullWidth?: boolean;
};

const variants: Record<ButtonVariant, string> = {
  primary:
    "bg-gradient-to-r from-primary to-primary-dark text-white shadow-card hover:shadow-glow border border-transparent",
  secondary: "border border-white/30 text-white bg-white/10 hover:bg-white/20",
  danger: "border border-red-600 bg-red-600 text-white hover:bg-red-700",
  ghost: "border border-slate-300 text-slate-700 bg-white hover:bg-slate-50",
  porter: "border border-porter-border bg-porter-surface-elev text-[#EAF4F9] hover:bg-porter-surface",
};

export function Button({ variant = "primary", fullWidth, className, ...props }: Props) {
  return (
    <button
      className={cx(
        "inline-flex min-h-[44px] items-center justify-center rounded-full px-6 py-3 text-sm font-semibold transition focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 focus-visible:ring-offset-transparent disabled:pointer-events-none",
        variants[variant],
        fullWidth && "w-full",
        props.disabled && "cursor-not-allowed opacity-50",
        className,
      )}
      {...props}
    />
  );
}
