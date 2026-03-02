import { create } from "zustand";

import type { User } from "@/lib/types";

type AuthState = {
  token: string | null;
  user: User | null;
  isReady: boolean;
  setSession: (token: string, user: User) => void;
  clearSession: () => void;
  setReady: (value: boolean) => void;
};

export const useAuthStore = create<AuthState>((set) => ({
  token: null,
  user: null,
  isReady: false,
  setSession: (token, user) => set({ token, user }),
  clearSession: () => set({ token: null, user: null }),
  setReady: (value) => set({ isReady: value }),
}));