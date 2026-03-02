"use client";

import { useEffect, useId, useRef, type ReactNode } from "react";

import { cx } from "@/lib/utils";

type Props = {
  isOpen: boolean;
  title?: string;
  children: ReactNode;
  onClose: () => void;
  maxWidthClassName?: string;
};

export function Modal({ isOpen, title, children, onClose, maxWidthClassName }: Props) {
  const titleId = useId();
  const previousOverflow = useRef<string | null>(null);

  useEffect(() => {
    if (!isOpen) {
      return;
    }

    previousOverflow.current = document.body.style.overflow;
    document.body.style.overflow = "hidden";

    const onEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        onClose();
      }
    };

    window.addEventListener("keydown", onEscape);

    return () => {
      window.removeEventListener("keydown", onEscape);
      document.body.style.overflow = previousOverflow.current ?? "";
    };
  }, [isOpen, onClose]);

  if (!isOpen) {
    return null;
  }

  return (
    <div
      className="fixed inset-0 z-[400] flex items-center justify-center bg-black/50 p-4 backdrop-blur-sm"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget) {
          onClose();
        }
      }}
    >
      <div
        aria-labelledby={title ? titleId : undefined}
        aria-modal="true"
        className={cx(
          "w-full rounded-lg border border-slate-200 bg-white p-6 shadow-lg",
          maxWidthClassName ?? "max-w-2xl",
        )}
        role="dialog"
      >
        <div className="mb-4 flex items-center justify-between">
          <h3 id={titleId} className="font-title text-xl font-semibold text-slate-900">
            {title}
          </h3>
          <button
            aria-label="Fermer"
            className="rounded-md px-2 py-1 text-slate-500 hover:bg-slate-100"
            onClick={onClose}
            type="button"
          >
            ×
          </button>
        </div>
        {children}
      </div>
    </div>
  );
}
