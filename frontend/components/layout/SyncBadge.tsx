"use client";

import { useOutbox } from "@/hooks/useOutbox";
import { useOutboxStore } from "@/stores/outboxStore";

export function SyncBadge() {
  const count = useOutboxStore((state) => state.items.length);
  const { syncNow } = useOutbox();

  if (count === 0) {
    return null;
  }

  return (
    <button
      className="rounded-full border border-amber-300 bg-amber-100 px-3 py-1 text-xs font-semibold text-amber-800"
      onClick={() => {
        syncNow().catch(() => undefined);
      }}
      type="button"
    >
      {count} operation(s) en attente - Synchroniser maintenant
    </button>
  );
}