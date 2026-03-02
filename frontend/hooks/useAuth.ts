"use client";

import { useMemo } from "react";

import { logout as logoutLib } from "@/lib/auth";
import { useAuthStore } from "@/stores/authStore";

export function useAuth() {
  const token = useAuthStore((state) => state.token);
  const user = useAuthStore((state) => state.user);
  const clearSession = useAuthStore((state) => state.clearSession);
  const isReady = useAuthStore((state) => state.isReady);

  return useMemo(
    () => ({
      token,
      user,
      isReady,
      isAuthenticated: Boolean(token && user),
      logout: () => {
        logoutLib();
        clearSession();
      },
    }),
    [clearSession, isReady, token, user],
  );
}