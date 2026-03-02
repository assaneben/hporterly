"use client";

import { cx } from "@/lib/utils";

export type ToastItem = {
  id: string;
  title: string;
  message?: string;
  type?: "info" | "success" | "warning" | "error";
};

const typeStyles = {
  info: "border-l-blue-500",
  success: "border-l-emerald-500",
  warning: "border-l-amber-500",
  error: "border-l-red-500",
} as const;

export function Toast({ items, onDismiss }: { items: ToastItem[]; onDismiss: (id: string) => void }) {
  return (
    <div className="fixed bottom-4 right-4 z-[500] flex w-full max-w-sm flex-col gap-2">
      {items.map((item) => (
        <div
          key={item.id}
          className={cx(
            "rounded-md border border-slate-200 border-l-4 bg-white p-4 shadow-card",
            typeStyles[item.type ?? "info"],
          )}
          role="status"
          aria-live={item.type === "error" ? "assertive" : "polite"}
        >
          <div className="flex items-start justify-between gap-2">
            <div>
              <p className="text-sm font-semibold text-slate-900">{item.title}</p>
              {item.message ? <p className="text-sm text-slate-600">{item.message}</p> : null}
            </div>
            <button
              aria-label="Fermer le toast"
              className="text-slate-500"
              onClick={() => onDismiss(item.id)}
              type="button"
            >
              ×
            </button>
          </div>
        </div>
      ))}
    </div>
  );
}
