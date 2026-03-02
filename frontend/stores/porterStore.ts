import { create } from "zustand";

import type { Porter } from "@/lib/types";

type PorterState = {
  porters: Porter[];
  setPorters: (porters: Porter[]) => void;
  updatePorter: (porter: Porter) => void;
};

export const usePorterStore = create<PorterState>((set, get) => ({
  porters: [],
  setPorters: (porters) => set({ porters }),
  updatePorter: (porter) => {
    const existing = get().porters;
    const index = existing.findIndex((item) => item.id === porter.id);
    if (index < 0) {
      set({ porters: [porter, ...existing] });
      return;
    }

    const next = [...existing];
    next[index] = porter;
    set({ porters: next });
  },
}));