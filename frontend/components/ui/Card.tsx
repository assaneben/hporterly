import { cx } from "@/lib/utils";
import type { HTMLAttributes } from "react";

type Props = HTMLAttributes<HTMLDivElement> & {
  hoverable?: boolean;
  variant?: "glass" | "dashboard" | "porter";
};

const variantClass: Record<NonNullable<Props["variant"]>, string> = {
  glass: "border border-white/20 bg-white/15 backdrop-blur-xl shadow-glass text-white",
  dashboard: "border border-slate-200 bg-white shadow-sm text-slate-900",
  porter: "border border-[#1F6A8A] bg-[#012A4A] text-[#EAF4F9]",
};

export function Card({ className, hoverable, variant = "dashboard", ...props }: Props) {
  return (
    <div
      className={cx(
        "rounded-lg p-6 transition",
        variantClass[variant],
        hoverable && "hover:-translate-y-1 hover:shadow-glow",
        className,
      )}
      {...props}
    />
  );
}