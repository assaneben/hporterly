"use client";

import { useCallback } from "react";

import { api } from "@/lib/api";
import type { NotificationItem } from "@/lib/types";
import { useNotificationStore } from "@/stores/notificationStore";

export function useNotifications() {
  const notifications = useNotificationStore((state) => state.notifications);
  const unreadCount = useNotificationStore((state) => state.unreadCount);
  const setNotifications = useNotificationStore((state) => state.setNotifications);
  const setUnreadCount = useNotificationStore((state) => state.setUnreadCount);

  const fetchNotifications = useCallback(async () => {
    const data = await api.get<NotificationItem[]>("/notifications?limit=50");
    setNotifications(data);
  }, [setNotifications]);

  const fetchUnreadCount = useCallback(async () => {
    const data = await api.get<{ count: number }>("/notifications/unread-count");
    setUnreadCount(data.count);
  }, [setUnreadCount]);

  const markAllRead = useCallback(async () => {
    await api.post<void>("/notifications/mark-all-read");
    await fetchNotifications();
  }, [fetchNotifications]);

  return {
    notifications,
    unreadCount,
    fetchNotifications,
    fetchUnreadCount,
    markAllRead,
  };
}