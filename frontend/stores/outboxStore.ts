import { create } from "zustand";

import type { OutboxItem } from "@/lib/outbox";

type OutboxState = {
  items: OutboxItem[];
  setItems: (items: OutboxItem[]) => void;
  count: () => number;
};

export const useOutboxStore = create<OutboxState>((set, get) => ({
  items: [],
  setItems: (items) => set({ items }),
  count: () => get().items.length,
}));