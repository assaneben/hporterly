import { cx } from "@/lib/utils";

type Props = {
  title: string;
  description?: string;
  className?: string;
};

export function EmptyState({ title, description, className }: Props) {
  return (
    <div
      className={cx(
        "rounded-lg border border-dashed border-slate-300 bg-slate-50/70 px-4 py-6 text-center text-slate-800",
        className,
      )}
      role="status"
    >
      <p className="font-title text-lg font-semibold">{title}</p>
      {description ? <p className="mt-1 text-sm opacity-80">{description}</p> : null}
    </div>
  );
}
