import { cx } from "@/lib/utils";

type BadgeVariant =
  | "n1"
  | "n2"
  | "n3"
  | "n4"
  | "pending"
  | "assigned"
  | "in_progress"
  | "completed"
  | "canceled"
  | "paused";

const classes: Record<BadgeVariant, string> = {
  n1: "bg-red-50 text-red-600 border-red-300",
  n2: "bg-orange-50 text-orange-600 border-orange-200",
  n3: "bg-blue-50 text-blue-600 border-blue-300",
  n4: "bg-emerald-50 text-emerald-700 border-emerald-300",
  pending: "bg-orange-50 text-orange-700 border-orange-200",
  assigned: "bg-violet-50 text-violet-700 border-violet-300",
  in_progress: "bg-blue-100 text-blue-700 border-blue-300",
  completed: "bg-green-100 text-green-700 border-green-300",
  canceled: "bg-red-50 text-red-700 border-red-300",
  paused: "bg-amber-100 text-amber-700 border-amber-300",
};

type Props = {
  children: string;
  variant: BadgeVariant;
  pulse?: boolean;
  className?: string;
};

export function Badge({ children, variant, pulse, className }: Props) {
  return (
    <span
      className={cx(
        "inline-flex items-center rounded-full border px-3 py-1 text-xs font-semibold",
        classes[variant],
        pulse && "animate-hplyPulse",
        className,
      )}
    >
      {children}
    </span>
  );
}