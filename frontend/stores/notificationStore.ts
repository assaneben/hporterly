import { create } from "zustand";

import type { NotificationItem } from "@/lib/types";

type NotificationState = {
  notifications: NotificationItem[];
  unreadCount: number;
  setNotifications: (notifications: NotificationItem[]) => void;
  setUnreadCount: (count: number) => void;
};

export const useNotificationStore = create<NotificationState>((set) => ({
  notifications: [],
  unreadCount: 0,
  setNotifications: (notifications) =>
    set({
      notifications,
      unreadCount: notifications.filter((item) => !item.isRead).length,
    }),
  setUnreadCount: (count) => set({ unreadCount: count }),
}));