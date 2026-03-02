"use client";

import { useCallback, useEffect, useRef, useState } from "react";

import { EmptyState } from "@/components/ui/EmptyState";
import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { useNotifications } from "@/hooks/useNotifications";
import { formatDate } from "@/lib/utils";

export function NotificationBell() {
  const { notifications, unreadCount, fetchNotifications, fetchUnreadCount, markAllRead } = useNotifications();
  const [open, setOpen] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const rootRef = useRef<HTMLDivElement | null>(null);

  const refreshUnread = useCallback(() => {
    fetchUnreadCount().catch(() => undefined);
  }, [fetchUnreadCount]);

  const loadPanel = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      await fetchNotifications();
      await fetchUnreadCount();
    } catch {
      setError("Impossible de charger les notifications.");
    } finally {
      setLoading(false);
    }
  }, [fetchNotifications, fetchUnreadCount]);

  useEffect(() => {
    refreshUnread();
    const id = setInterval(refreshUnread, 15_000);
    return () => clearInterval(id);
  }, [refreshUnread]);

  useEffect(() => {
    if (!open) {
      return;
    }

    loadPanel().catch(() => undefined);
  }, [open, loadPanel]);

  useEffect(() => {
    if (!open) {
      return;
    }

    const onPointerDown = (event: MouseEvent) => {
      if (!rootRef.current?.contains(event.target as Node)) {
        setOpen(false);
      }
    };

    window.addEventListener("mousedown", onPointerDown);
    return () => window.removeEventListener("mousedown", onPointerDown);
  }, [open]);

  return (
    <div className="relative" ref={rootRef}>
      <button
        data-testid="notification-bell"
        aria-expanded={open}
        aria-haspopup="dialog"
        aria-label="Notifications"
        className="relative rounded-full bg-white/20 px-3 py-2 text-white transition hover:-translate-y-px hover:bg-white/30"
        onClick={() => setOpen((value) => !value)}
        type="button"
      >
        Bell
        {unreadCount > 0 ? (
          <span
            data-testid="notification-unread-badge"
            className="absolute -right-1 -top-1 rounded-full bg-red-500 px-1.5 text-[10px] font-semibold text-white"
          >
            {unreadCount}
          </span>
        ) : null}
      </button>

      {open ? (
        <div
          data-testid="notification-panel"
          className="hply-fade-in absolute right-0 z-[320] mt-2 w-[340px] rounded-xl border border-slate-200 bg-white p-3 text-slate-900 shadow-xl"
          role="dialog"
        >
          <div className="mb-2 flex items-center justify-between gap-2">
            <h3 className="font-title text-base font-semibold">Notifications</h3>
            <button
              data-testid="notification-mark-all"
              className="rounded-md border border-slate-300 px-2 py-1 text-xs font-semibold text-slate-700 hover:bg-slate-50 disabled:opacity-50"
              disabled={notifications.length === 0}
              onClick={() => {
                markAllRead()
                  .then(() => fetchUnreadCount())
                  .catch(() => setError("Impossible de marquer les messages comme lus."));
              }}
              type="button"
            >
              Tout lire
            </button>
          </div>

          {error ? <InlineFeedback message={error} tone="error" /> : null}

          {loading ? (
            <p className="py-6 text-center text-sm text-slate-500">Chargement...</p>
          ) : notifications.length === 0 ? (
            <EmptyState
              title="Aucune notification"
              description="Les nouveaux messages et alertes apparaitront ici."
              className="px-3 py-5"
            />
          ) : (
            <div className="hply-soft-scroll max-h-80 space-y-2 overflow-y-auto pr-1">
              {notifications.map((item) => (
                <article
                  key={item.id}
                  className={`rounded-lg border px-2.5 py-2 ${item.isRead ? "border-slate-200 bg-slate-50" : "border-sky-200 bg-sky-50"}`}
                >
                  <p className="text-[11px] font-semibold uppercase tracking-[0.08em] text-slate-500">{item.title}</p>
                  <p className="mt-1 text-sm text-slate-800">{item.message}</p>
                  <p className="mt-1 text-[11px] text-slate-500">{formatDate(item.createdAt)}</p>
                </article>
              ))}
            </div>
          )}
        </div>
      ) : null}
    </div>
  );
}
