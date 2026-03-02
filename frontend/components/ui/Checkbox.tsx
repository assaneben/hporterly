import type { InputHTMLAttributes } from "react";

import { cx } from "@/lib/utils";

type Props = Omit<InputHTMLAttributes<HTMLInputElement>, "type"> & {
  label: string;
};

export function Checkbox({ label, className, ...props }: Props) {
  return (
    <label className={cx("flex min-h-[44px] items-center gap-2 text-sm text-slate-700", className)}>
      <input type="checkbox" className="h-4 w-4 rounded border-slate-300" {...props} />
      <span>{label}</span>
    </label>
  );
}