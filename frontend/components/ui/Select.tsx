import type { SelectHTMLAttributes } from "react";

import { cx } from "@/lib/utils";

type Props = SelectHTMLAttributes<HTMLSelectElement> & {
  label?: string;
};

export function Select({ label, className, children, ...props }: Props) {
  return (
    <label className="block">
      {label ? <span className="mb-1 block text-sm font-medium text-slate-700">{label}</span> : null}
      <select
        className={cx(
          "min-h-[44px] w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-slate-900",
          className,
        )}
        {...props}
      >
        {children}
      </select>
    </label>
  );
}