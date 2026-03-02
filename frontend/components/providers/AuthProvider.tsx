"use client";

import { useEffect } from "react";
import { usePathname, useRouter } from "next/navigation";

import { getRoleHome, restoreAuth } from "@/lib/auth";
import { useAuthStore } from "@/stores/authStore";

const PUBLIC_PATHS = ["/login"];

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const router = useRouter();
  const pathname = usePathname();
  const user = useAuthStore((state) => state.user);
  const setSession = useAuthStore((state) => state.setSession);
  const clearSession = useAuthStore((state) => state.clearSession);
  const isReady = useAuthStore((state) => state.isReady);
  const setReady = useAuthStore((state) => state.setReady);

  useEffect(() => {
    const restored = restoreAuth();

    if (restored.token && restored.user) {
      setSession(restored.token, restored.user);
    } else {
      clearSession();
    }

    setReady(true);
  }, [clearSession, setReady, setSession]);

  useEffect(() => {
    if (!isReady) {
      return;
    }

    const isPublic = PUBLIC_PATHS.some((path) => pathname.startsWith(path));

    if (!user && !isPublic) {
      router.replace("/login");
      return;
    }

    if (user && pathname === "/") {
      router.replace(getRoleHome(user.role));
    }
  }, [isReady, pathname, router, user]);

  return <>{children}</>;
}