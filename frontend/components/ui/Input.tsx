import type { InputHTMLAttributes } from "react";

import { cx } from "@/lib/utils";

type Props = InputHTMLAttributes<HTMLInputElement> & {
  label?: string;
  error?: string;
  labelClassName?: string;
  containerClassName?: string;
};

export function Input({
  id,
  label,
  error,
  className,
  labelClassName,
  containerClassName,
  ...props
}: Props) {
  return (
    <label className={cx("block", containerClassName)}>
      {label ? (
        <span className={cx("mb-1 block text-sm font-medium text-slate-700", labelClassName)}>{label}</span>
      ) : null}
      <input
        id={id}
        className={cx(
          "min-h-[44px] w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-slate-900 placeholder:text-slate-400",
          error && "border-red-500",
          className,
        )}
        {...props}
      />
      {error ? <span className="mt-1 block text-xs text-red-600">{error}</span> : null}
    </label>
  );
}