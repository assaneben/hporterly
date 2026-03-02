"use client";

import { useCallback, useEffect, useRef } from "react";

import { apiRequest } from "@/lib/api";
import { enqueueOutbox, loadOutbox, processOutbox } from "@/lib/outbox";
import { useAuthStore } from "@/stores/authStore";
import { useOutboxStore } from "@/stores/outboxStore";

export function useOutbox() {
  const user = useAuthStore((state) => state.user);
  const setItems = useOutboxStore((state) => state.setItems);
  const heartbeatRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const refresh = useCallback(() => {
    setItems(loadOutbox().items);
  }, [setItems]);

  const syncNow = useCallback(async () => {
    if (!user) {
      return;
    }

    const next = await processOutbox({
      ownerUserId: user.id,
      runRequest: async (item) => {
        await apiRequest(item.path, {
          method: item.method,
          body: item.body,
        });
      },
    });

    setItems(next.items);
  }, [setItems, user]);

  const queueMutation = useCallback(
    (entry: {
      type: string;
      path: string;
      method: "POST" | "PATCH" | "DELETE";
      body: Record<string, unknown>;
      meta?: { optimistic_id?: string };
      queue_only_on_network?: boolean;
    }) => {
      if (!user) {
        throw new Error("Utilisateur non authentifie");
      }

      const payload = enqueueOutbox({
        id: crypto.randomUUID(),
        type: entry.type,
        path: entry.path,
        method: entry.method,
        body: entry.body,
        meta: entry.meta,
        created_at: new Date().toISOString(),
        owner_user_id: user.id,
        queue_only_on_network: entry.queue_only_on_network,
      });

      setItems(payload.items);
    },
    [setItems, user],
  );

  useEffect(() => {
    refresh();
  }, [refresh]);

  useEffect(() => {
    const onSyncTrigger = () => {
      syncNow().catch(() => undefined);
    };

    window.addEventListener("online", onSyncTrigger);
    window.addEventListener("focus", onSyncTrigger);

    const onVisibility = () => {
      if (document.visibilityState === "visible") {
        onSyncTrigger();
      }
    };

    document.addEventListener("visibilitychange", onVisibility);

    heartbeatRef.current = setInterval(() => {
      onSyncTrigger();
    }, 15_000);

    return () => {
      window.removeEventListener("online", onSyncTrigger);
      window.removeEventListener("focus", onSyncTrigger);
      document.removeEventListener("visibilitychange", onVisibility);
      if (heartbeatRef.current) {
        clearInterval(heartbeatRef.current);
      }
    };
  }, [syncNow]);

  return {
    queueMutation,
    syncNow,
    refresh,
  };
}